use crate::from_wasm::*;
use makepad_live_id::*;

pub struct WasmJSOutputFn {
    pub name: String,
    pub body: String,
    pub temp: usize,
}

pub struct WasmJSOutput {
    pub temp_alloc: usize,
    pub fns: Vec<WasmJSOutputFn>,
}

impl WasmJSOutput {
    pub fn alloc_temp(&mut self) -> usize {
        self.temp_alloc += 1;
        self.temp_alloc
    }

    pub fn check_slot(
        &mut self,
        slot: usize,
        is_recur: bool,
        prop: &str,
        temp: usize,
        name: &str,
    ) -> Option<usize> {
        // ok so if we recur
        if is_recur {
            // call body
            self.push_ln(slot, &format!("{}({});", name, prop));
            // check if we already have the fn
            if self.fns.iter().any(|p| p.name == name) {
                return None;
            }
            self.fns.push(WasmJSOutputFn {
                name: name.to_string(),
                body: String::new(),
                temp,
            });
            Some(self.fns.len() - 1)
        } else {
            self.push_ln(slot, &format!("let t{} = {};", temp, prop));
            Some(slot)
        }
    }

    pub fn push_ln(&mut self, slot: usize, s: &str) {
        self.fns[slot].body.push_str(s);
        self.fns[slot].body.push('\n');
    }
}

pub trait ToWasm {
    fn u32_size() -> usize;

    fn type_name() -> &'static str {
        panic!()
    }
    fn live_id() -> LiveId {
        panic!()
    }

    fn read_to_wasm(inp: &mut ToWasmMsgRef) -> Self;

    fn to_wasm_js_body(
        out: &mut WasmJSOutput,
        slot: usize,
        is_recur: bool,
        prop: &str,
        temp: usize,
    );

    fn to_js_code() -> String {
        let mut wrapper = String::new();
        let id = Self::live_id();
        wrapper.push_str(&format!("{}(t0){{\n", Self::type_name()));
        wrapper.push_str("let app = this.app;\n");
        wrapper.push_str(&format!("this.reserve_u32({});\n", 4 + Self::u32_size()));
        wrapper.push_str(&format!(
            "app.u32[this.u32_offset ++] = {};\n",
            id.0 & 0xffff_ffff
        ));
        wrapper.push_str(&format!(
            "app.u32[this.u32_offset ++] = {};\n",
            (id.0 >> 32)
        ));
        wrapper.push_str("let block_len_offset = this.u32_offset ++ - this.u32_ptr;\n\n");

        let mut out = WasmJSOutput {
            temp_alloc: 0,
            fns: vec![WasmJSOutputFn {
                name: String::new(),
                body: String::new(),
                temp: 0,
            }],
        };

        let new_temp = out.alloc_temp();
        Self::to_wasm_js_body(&mut out, 0, false, "t0", new_temp);

        for p in out.fns.iter().rev() {
            if p.name.is_empty() {
                wrapper.push_str(&p.body);
            } else {
                wrapper.push_str(&format!(
                    "let {} = (t{})=>{{\n{}}}\n",
                    p.name, p.temp, p.body
                ))
            }
        }

        wrapper.push_str("if( (this.u32_offset & 1) != 0){ app.u32[this.u32_offset ++] = 0;}\n");
        wrapper.push_str("let new_len = (this.u32_offset - this.u32_ptr) >> 1;\n");
        wrapper.push_str("app.u32[this.u32_ptr + block_len_offset] = new_len;\n");
        wrapper.push_str("app.u32[this.u32_ptr + 1] = new_len;\n");
        wrapper.push_str("}\n");
        wrapper
    }
}

#[derive(Clone, Default, Debug)]
pub struct ToWasmMsg {
    data: Vec<u64>,
}

/// End of a block, as the message length in u64 up to and including it. The
/// JS writer stamps it into every block header so a handler that reads fewer
/// fields than the block carries still leaves the cursor on the next block.
pub struct ToWasmBlockSkip {
    len: usize,
}

#[derive(Clone, Default, Debug)]
pub struct ToWasmMsgRef<'a> {
    data: &'a [u64],
    pub u32_offset: usize,
}

impl ToWasmMsg {
    pub fn take_ownership(val: u32) -> Self {
        unsafe {
            let ptr = val as *mut u64;
            let head = ptr.offset(0).read();
            let len = (head >> 32) as usize;
            let cap = (head & 0xffff_ffff) as usize;

            Self {
                data: Vec::from_raw_parts(ptr, len, cap),
                //u32_offset: 2,
            }
        }
    }

    pub fn into_from_wasm(self) -> FromWasmMsg {
        FromWasmMsg {
            data: self.data,
            odd: false,
        }
    }

    pub fn as_ref(&self) -> ToWasmMsgRef<'_> {
        ToWasmMsgRef {
            data: &self.data,
            u32_offset: 2,
        }
    }

    pub fn as_ref_at(&self, offset: usize) -> ToWasmMsgRef<'_> {
        ToWasmMsgRef {
            data: &self.data,
            u32_offset: offset,
        }
    }
}

impl<'a> ToWasmMsgRef<'a> {
    pub fn read_u32(&mut self) -> u32 {
        let ret = if self.u32_offset & 1 != 0 {
            (self.data[self.u32_offset >> 1] >> 32) as u32
        } else {
            (self.data[self.u32_offset >> 1] & 0xffff_ffff) as u32
        };
        self.u32_offset += 1;
        ret
    }

    pub fn read_block_skip(&mut self) -> ToWasmBlockSkip {
        ToWasmBlockSkip {
            len: self.read_u32() as usize,
        }
    }

    pub fn block_skip(&mut self, block_skip: ToWasmBlockSkip) {
        self.u32_offset = block_skip.len << 1
    }

    pub fn read_f32(&mut self) -> f32 {
        f32::from_bits(self.read_u32())
    }

    pub fn read_u64(&mut self) -> u64 {
        self.u32_offset += self.u32_offset & 1;
        let ret = self.data[self.u32_offset >> 1];
        self.u32_offset += 2;
        ret
    }

    pub fn read_f64(&mut self) -> f64 {
        f64::from_bits(self.read_u64())
    }

    pub fn read_string(&mut self) -> String {
        let chars = self.read_u32();
        let mut out = String::new();
        for _ in 0..chars {
            out.push(char::from_u32(self.read_u32()).unwrap_or('?'));
        }
        out
    }

    pub fn was_last_block(&mut self) -> bool {
        self.u32_offset += self.u32_offset & 1;
        self.u32_offset >> 1 >= self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Lays out blocks exactly like the JS in `ToWasm::to_js_code`: two u32 of
    /// live id, one u32 holding the message length up to the end of the block,
    /// the body, then padding to a u64 boundary.
    fn encode(blocks: &[(u64, &[u32])]) -> Vec<u64> {
        let mut words: Vec<u32> = vec![0, 0];
        for (id, body) in blocks {
            words.push(*id as u32);
            words.push((*id >> 32) as u32);
            let len_slot = words.len();
            words.push(0);
            words.extend_from_slice(body);
            if words.len() & 1 != 0 {
                words.push(0);
            }
            let len = (words.len() >> 1) as u32;
            words[len_slot] = len;
            words[1] = len;
        }
        words
            .chunks(2)
            .map(|pair| pair[0] as u64 | (pair[1] as u64) << 32)
            .collect()
    }

    /// Mirrors the dispatch loop in the web backend: read the id, take the
    /// block skip, let the handler consume `read` words, then skip.
    fn dispatch(data: &[u64], read: usize) -> Vec<(u64, Vec<u32>)> {
        let mut msg = ToWasmMsgRef {
            data,
            u32_offset: 2,
        };
        let mut out = Vec::new();
        while !msg.was_last_block() {
            let id = msg.read_u64();
            let skip = msg.read_block_skip();
            let body = (0..read).map(|_| msg.read_u32()).collect();
            msg.block_skip(skip);
            out.push((id, body));
        }
        out
    }

    #[test]
    fn one_block_is_read_back() {
        let data = encode(&[(7, &[11, 12, 13])]);
        assert_eq!(dispatch(&data, 3), vec![(7, vec![11, 12, 13])]);
    }

    #[test]
    fn every_block_of_a_batch_is_read_back() {
        let data = encode(&[(7, &[11, 12, 13]), (8, &[21, 22, 23])]);
        assert_eq!(
            dispatch(&data, 3),
            vec![(7, vec![11, 12, 13]), (8, vec![21, 22, 23])]
        );
    }

    #[test]
    fn a_block_the_handler_ignored_is_skipped() {
        let data = encode(&[(7, &[11]), (8, &[21, 22]), (9, &[])]);
        assert_eq!(
            dispatch(&data, 0),
            vec![(7, vec![]), (8, vec![]), (9, vec![])]
        );
    }
}
