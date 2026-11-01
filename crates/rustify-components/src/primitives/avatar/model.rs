use super::ImageLoadingStatus;

#[cfg_attr(not(any(target_arch = "wasm32", test)), allow(dead_code))]
#[derive(Clone, Debug)]
pub(super) struct Request {
    pub source: String,
    pub generation: u64,
    pub status: ImageLoadingStatus,
}
#[cfg_attr(not(any(target_arch = "wasm32", test)), allow(dead_code))]
impl Request {
    pub fn new(source: String) -> Self {
        Self {
            source,
            generation: 0,
            status: ImageLoadingStatus::Idle,
        }
    }
    pub fn set_source(&mut self, source: String) {
        if self.source == source {
            return;
        }
        self.source = source;
        self.generation = self.generation.wrapping_add(1);
        self.status = if self.source.is_empty() {
            ImageLoadingStatus::Error
        } else {
            ImageLoadingStatus::Loading
        };
    }
    pub fn inspect(&mut self, generation: u64, complete: bool, natural_width: u32) {
        if generation != self.generation {
            return;
        }
        self.status = if self.source.is_empty() {
            ImageLoadingStatus::Error
        } else if !complete {
            ImageLoadingStatus::Loading
        } else if natural_width > 0 {
            ImageLoadingStatus::Loaded
        } else {
            ImageLoadingStatus::Error
        };
    }
    pub fn fail(&mut self, generation: u64) {
        if generation == self.generation {
            self.status = ImageLoadingStatus::Error;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn old_image_events_cannot_replace_the_current_source_result() {
        let mut request = Request::new("/first.png".into());
        let first = request.generation;
        request.inspect(first, false, 0);
        request.set_source("/second.png".into());
        let second = request.generation;
        request.inspect(second, true, 32);
        request.fail(first);
        request.inspect(first, true, 0);
        assert_eq!(request.status, ImageLoadingStatus::Loaded);
        assert_eq!(request.source, "/second.png");
    }
    #[test]
    fn cached_attachment_distinguishes_a_loaded_image_from_an_error() {
        let mut loaded = Request::new("/cached.png".into());
        loaded.inspect(0, true, 24);
        assert_eq!(loaded.status, ImageLoadingStatus::Loaded);
        let mut broken = Request::new("/broken.png".into());
        broken.inspect(0, true, 0);
        assert_eq!(broken.status, ImageLoadingStatus::Error);
    }
    #[test]
    fn a_changed_source_restarts_loading_but_an_unchanged_source_does_not() {
        let mut request = Request::new("/old.png".into());
        request.inspect(0, true, 24);
        request.set_source("/new.png".into());
        assert_eq!(request.status, ImageLoadingStatus::Loading);
        let generation = request.generation;
        request.inspect(generation, true, 24);
        request.set_source("/new.png".into());
        assert_eq!(request.generation, generation);
        assert_eq!(request.status, ImageLoadingStatus::Loaded);
    }
    #[test]
    fn an_absent_url_is_an_error_after_attachment_without_a_request() {
        let mut request = Request::new(String::new());
        assert_eq!(request.status, ImageLoadingStatus::Idle);
        request.inspect(0, false, 0);
        assert_eq!(request.status, ImageLoadingStatus::Error);
    }
}
