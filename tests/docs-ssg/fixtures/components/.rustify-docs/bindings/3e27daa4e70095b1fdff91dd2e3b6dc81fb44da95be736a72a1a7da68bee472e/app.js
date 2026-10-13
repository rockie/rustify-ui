import { create_dom_host } from './snippets/rustify-ui-4db2f10f10788f4c/web/dom-runtime.js';


export class IntoUnderlyingByteSource {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        IntoUnderlyingByteSourceFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_intounderlyingbytesource_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get autoAllocateChunkSize() {
        const ret = wasm.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr);
        return ret >>> 0;
    }
    cancel() {
        const ptr = this.__destroy_into_raw();
        wasm.intounderlyingbytesource_cancel(ptr);
    }
    /**
     * @param {ReadableByteStreamController} controller
     * @returns {Promise<any>}
     */
    pull(controller) {
        const ret = wasm.intounderlyingbytesource_pull(this.__wbg_ptr, controller);
        return ret;
    }
    /**
     * @param {ReadableByteStreamController} controller
     */
    start(controller) {
        wasm.intounderlyingbytesource_start(this.__wbg_ptr, controller);
    }
    /**
     * @returns {ReadableStreamType}
     */
    get type() {
        const ret = wasm.intounderlyingbytesource_type(this.__wbg_ptr);
        return __wbindgen_enum_ReadableStreamType[ret];
    }
}
if (Symbol.dispose) IntoUnderlyingByteSource.prototype[Symbol.dispose] = IntoUnderlyingByteSource.prototype.free;

export class IntoUnderlyingSink {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        IntoUnderlyingSinkFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_intounderlyingsink_free(ptr, 0);
    }
    /**
     * @param {any} reason
     * @returns {Promise<any>}
     */
    abort(reason) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.intounderlyingsink_abort(ptr, reason);
        return ret;
    }
    /**
     * @returns {Promise<any>}
     */
    close() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.intounderlyingsink_close(ptr);
        return ret;
    }
    /**
     * @param {any} chunk
     * @returns {Promise<any>}
     */
    write(chunk) {
        const ret = wasm.intounderlyingsink_write(this.__wbg_ptr, chunk);
        return ret;
    }
}
if (Symbol.dispose) IntoUnderlyingSink.prototype[Symbol.dispose] = IntoUnderlyingSink.prototype.free;

export class IntoUnderlyingSource {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        IntoUnderlyingSourceFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_intounderlyingsource_free(ptr, 0);
    }
    cancel() {
        const ptr = this.__destroy_into_raw();
        wasm.intounderlyingsource_cancel(ptr);
    }
    /**
     * @param {ReadableStreamDefaultController} controller
     * @returns {Promise<any>}
     */
    pull(controller) {
        const ret = wasm.intounderlyingsource_pull(this.__wbg_ptr, controller);
        return ret;
    }
}
if (Symbol.dispose) IntoUnderlyingSource.prototype[Symbol.dispose] = IntoUnderlyingSource.prototype.free;

export function docs_hydrate() {
    const ret = wasm.docs_hydrate();
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * A DOM host for instance `instance`, from the JS module this crate ships.
 *
 * Created here rather than by the loader because the module is a
 * wasm-bindgen snippet: its path is the bindgen output's to decide, and
 * the glue already imports it. The host holds no state between calls, so
 * every instance gets one of its own.
 * @param {number} instance
 * @returns {any}
 */
export function rustify_dom_create_host(instance) {
    const ret = wasm.rustify_dom_create_host(instance);
    return ret;
}

/**
 * Starts this instance's runtime with the hooks its loader created.
 *
 * Refused when the hooks speak another protocol, or when the instance has
 * already started: one instance has one host, and a second would leave
 * the first one's listeners without the signal that ends them.
 * @param {any} hooks
 */
export function rustify_runtime_boot(hooks) {
    const ret = wasm.rustify_runtime_boot(hooks);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * The protocol this build speaks, for the loader to compare with its own
 * before it creates anything.
 * @returns {number}
 */
export function rustify_runtime_protocol() {
    const ret = wasm.rustify_runtime_protocol();
    return ret >>> 0;
}
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_debug_string_0e68cf47c9cbd9b0: function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_is_falsy_4502df4d571fca70: function(arg0) {
            const ret = !arg0;
            return ret;
        },
        __wbg___wbindgen_is_function_fcda5e3902d732fe: function(arg0) {
            const ret = typeof(arg0) === 'function';
            return ret;
        },
        __wbg___wbindgen_is_null_5160b3e381865372: function(arg0) {
            const ret = arg0 === null;
            return ret;
        },
        __wbg___wbindgen_is_undefined_8c687d0b90d5b524: function(arg0) {
            const ret = arg0 === undefined;
            return ret;
        },
        __wbg___wbindgen_jsval_eq_9fdcd3c0a860dd3b: function(arg0, arg1) {
            const ret = arg0 === arg1;
            return ret;
        },
        __wbg___wbindgen_number_get_1dc732b810cb937c: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'number' ? obj : undefined;
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_rethrow_dba7bb2caa14ba21: function(arg0) {
            throw arg0;
        },
        __wbg___wbindgen_string_get_92ab86bb19cbc12f: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_5d9e815e6fdf150f: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg__wbg_cb_unref_997e73d32238e655: function(arg0) {
            arg0._wbg_cb_unref();
        },
        __wbg_aborted_523817079f8cb2c0: function(arg0) {
            const ret = arg0.aborted;
            return ret;
        },
        __wbg_activeElement_a40cd04e4e5cfaff: function(arg0) {
            const ret = arg0.activeElement;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_addEventListener_2f29f3aa43f18e91: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3, arg4);
        }, arguments); },
        __wbg_addEventListener_9534e2dc12044ee4: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3);
        }, arguments); },
        __wbg_add_8ead3105ef6d0bda: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.add(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_adoptedStyleSheets_6256ab69072e5f67: function(arg0) {
            const ret = arg0.adoptedStyleSheets;
            return ret;
        },
        __wbg_altKey_c75bf4903a5db4c2: function(arg0) {
            const ret = arg0.altKey;
            return ret;
        },
        __wbg_altKey_fa381deee4b395cd: function(arg0) {
            const ret = arg0.altKey;
            return ret;
        },
        __wbg_appendChild_0a61ee9a9da4556f: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.appendChild(arg1);
            return ret;
        }, arguments); },
        __wbg_attachShadow_0927b90832039f4c: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.attachShadow(arg1);
            return ret;
        }, arguments); },
        __wbg_bind_4eb5bd0f63c11d34: function(arg0, arg1, arg2, arg3) {
            const ret = arg0.bind(arg1, arg2, arg3);
            return ret;
        },
        __wbg_bind_e0ce24d39d63824d: function(arg0, arg1, arg2) {
            const ret = arg0.bind(arg1, arg2);
            return ret;
        },
        __wbg_body_7d83a19bffb260db: function(arg0) {
            const ret = arg0.body;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_buffer_4a989bded7035f57: function(arg0) {
            const ret = arg0.buffer;
            return ret;
        },
        __wbg_button_f6aa74c65b739064: function(arg0) {
            const ret = arg0.button;
            return ret;
        },
        __wbg_byobRequest_f161fc37241dd3d4: function(arg0) {
            const ret = arg0.byobRequest;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_byteLength_0ddb1795e2c7e689: function(arg0) {
            const ret = arg0.byteLength;
            return ret;
        },
        __wbg_byteOffset_46eb015f52b6ad7d: function(arg0) {
            const ret = arg0.byteOffset;
            return ret;
        },
        __wbg_call_269c5566fbede3eb: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.call(arg1);
            return ret;
        }, arguments); },
        __wbg_call_6bcf8d3e20937e46: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.call(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_cancelBubble_946b05eecc8e3059: function(arg0) {
            const ret = arg0.cancelBubble;
            return ret;
        },
        __wbg_checked_ab679020f304032a: function(arg0) {
            const ret = arg0.checked;
            return ret;
        },
        __wbg_children_3175c2b4b8e7ea31: function(arg0) {
            const ret = arg0.children;
            return ret;
        },
        __wbg_classList_1501fdb96a8c296c: function(arg0) {
            const ret = arg0.classList;
            return ret;
        },
        __wbg_clientHeight_8669994caeda015c: function(arg0) {
            const ret = arg0.clientHeight;
            return ret;
        },
        __wbg_clientWidth_669677d957e958c7: function(arg0) {
            const ret = arg0.clientWidth;
            return ret;
        },
        __wbg_clientX_997500aa23d483d1: function(arg0) {
            const ret = arg0.clientX;
            return ret;
        },
        __wbg_cloneNode_780b5626b746de33: function() { return handleError(function (arg0) {
            const ret = arg0.cloneNode();
            return ret;
        }, arguments); },
        __wbg_cloneNode_e4d149ec7cda3b2b: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.cloneNode(arg1 !== 0);
            return ret;
        }, arguments); },
        __wbg_close_22882088c136df25: function() { return handleError(function (arg0) {
            arg0.close();
        }, arguments); },
        __wbg_close_8b609460dd26e367: function() { return handleError(function (arg0) {
            arg0.close();
        }, arguments); },
        __wbg_closest_c70e5aadd2078e50: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.closest(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_compareDocumentPosition_0a26c7a06c29c7b9: function(arg0, arg1) {
            const ret = arg0.compareDocumentPosition(arg1);
            return ret;
        },
        __wbg_composedPath_b2dae8cc5065d452: function(arg0) {
            const ret = arg0.composedPath();
            return ret;
        },
        __wbg_contains_322c48b603c87d38: function(arg0, arg1) {
            const ret = arg0.contains(arg1);
            return ret;
        },
        __wbg_content_405aafcafb81b239: function(arg0) {
            const ret = arg0.content;
            return ret;
        },
        __wbg_createComment_2f844d723b100a86: function(arg0, arg1, arg2) {
            const ret = arg0.createComment(getStringFromWasm0(arg1, arg2));
            return ret;
        },
        __wbg_createElementNS_a45335f2f24f9ec5: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            const ret = arg0.createElementNS(arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
            return ret;
        }, arguments); },
        __wbg_createElement_b9024dc5ba95ac27: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.createElement(getStringFromWasm0(arg1, arg2));
            return ret;
        }, arguments); },
        __wbg_createTextNode_6d2db3073c3d6df6: function(arg0, arg1, arg2) {
            const ret = arg0.createTextNode(getStringFromWasm0(arg1, arg2));
            return ret;
        },
        __wbg_create_dom_host_43c6c932010183ef: function(arg0, arg1) {
            const ret = create_dom_host(arg0 >>> 0, arg1 >>> 0);
            return ret;
        },
        __wbg_cssRules_6455fd9078b68262: function() { return handleError(function (arg0) {
            const ret = arg0.cssRules;
            return ret;
        }, arguments); },
        __wbg_ctrlKey_785e802b8798af13: function(arg0) {
            const ret = arg0.ctrlKey;
            return ret;
        },
        __wbg_ctrlKey_ba07ac4b2ea90600: function(arg0) {
            const ret = arg0.ctrlKey;
            return ret;
        },
        __wbg_defaultPrevented_6532d5b02b675eeb: function(arg0) {
            const ret = arg0.defaultPrevented;
            return ret;
        },
        __wbg_defer_after_9351b21611553c55: function(arg0, arg1, arg2) {
            arg0.defer_after(arg1, arg2);
        },
        __wbg_deleteProperty_a40aedf449952ca0: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.deleteProperty(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_deleteRule_7d1a17694da12eae: function() { return handleError(function (arg0, arg1) {
            arg0.deleteRule(arg1 >>> 0);
        }, arguments); },
        __wbg_documentElement_02547989338a4499: function(arg0) {
            const ret = arg0.documentElement;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_document_c7f486c52d63d24e: function(arg0) {
            const ret = arg0.document;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_enqueue_6cb545d22db14f33: function() { return handleError(function (arg0, arg1) {
            arg0.enqueue(arg1);
        }, arguments); },
        __wbg_firstChild_73284a53bf252a8e: function(arg0) {
            const ret = arg0.firstChild;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_firstElementChild_c4359d5571e95230: function(arg0) {
            const ret = arg0.firstElementChild;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_focus_79e6ed3164b6185f: function() { return handleError(function (arg0, arg1) {
            arg0.focus(arg1);
        }, arguments); },
        __wbg_focus_c1b2d4962209010b: function() { return handleError(function (arg0) {
            arg0.focus();
        }, arguments); },
        __wbg_getAttribute_b12f8535a2cb5368: function(arg0, arg1, arg2, arg3) {
            const ret = arg1.getAttribute(getStringFromWasm0(arg2, arg3));
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_getBoundingClientRect_299f40f8641457e6: function(arg0) {
            const ret = arg0.getBoundingClientRect();
            return ret;
        },
        __wbg_getClientRects_71a6de4ad137700b: function(arg0) {
            const ret = arg0.getClientRects();
            return ret;
        },
        __wbg_getComputedStyle_89f9e5c4de4b7edf: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.getComputedStyle(arg1);
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_getElementById_ccc92d66acf76819: function(arg0, arg1, arg2) {
            const ret = arg0.getElementById(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_getPropertyValue_778f881c5d299730: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = arg1.getPropertyValue(getStringFromWasm0(arg2, arg3));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_get_989d0a1309644f2b: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_get_b1f0ab13c737f856: function(arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return ret;
        },
        __wbg_get_signal_a52dbd5537e80565: function(arg0) {
            const ret = arg0.signal;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_get_unchecked_363572bdd397d473: function(arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return ret;
        },
        __wbg_go_cab2baec0fc0dbd7: function() { return handleError(function (arg0, arg1) {
            arg0.go(arg1);
        }, arguments); },
        __wbg_hasAttribute_847347996c609fca: function(arg0, arg1, arg2) {
            const ret = arg0.hasAttribute(getStringFromWasm0(arg1, arg2));
            return ret;
        },
        __wbg_hasChildNodes_4dfc7c260451ddaf: function(arg0) {
            const ret = arg0.hasChildNodes();
            return ret;
        },
        __wbg_height_24586a23e4789d28: function(arg0) {
            const ret = arg0.height;
            return ret;
        },
        __wbg_history_9d7bf9676b7c736c: function() { return handleError(function (arg0) {
            const ret = arg0.history;
            return ret;
        }, arguments); },
        __wbg_host_8c713ce042f34258: function(arg0) {
            const ret = arg0.host;
            return ret;
        },
        __wbg_href_de709e84ed5de5d2: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.href;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_innerHeight_f0945dfd6f2d3a87: function() { return handleError(function (arg0) {
            const ret = arg0.innerHeight;
            return ret;
        }, arguments); },
        __wbg_innerWidth_1553ed0783afe5b4: function() { return handleError(function (arg0) {
            const ret = arg0.innerWidth;
            return ret;
        }, arguments); },
        __wbg_insertBefore_5b6fe047589f01ee: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.insertBefore(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_insertRule_844cf7bcddbf717c: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = arg0.insertRule(getStringFromWasm0(arg1, arg2), arg3 >>> 0);
            return ret;
        }, arguments); },
        __wbg_instanceof_Comment_48d7ba8e240f4547: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Comment;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_CssStyleRule_493bc03959b5357f: function(arg0) {
            let result;
            try {
                result = arg0 instanceof CSSStyleRule;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Element_c95c45635d03047e: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Element;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlElement_bd89ce6abf9dad44: function(arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlInputElement_086894dc9c868a64: function(arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLInputElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlTextAreaElement_632129eba25e4367: function(arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLTextAreaElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_KeyboardEvent_e7ef7527c166dc71: function(arg0) {
            let result;
            try {
                result = arg0 instanceof KeyboardEvent;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_ShadowRoot_909fd69aa6c757e6: function(arg0) {
            let result;
            try {
                result = arg0 instanceof ShadowRoot;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Text_996faaa15a996306: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Text;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Window_a3b8566f0a9c5d1a: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_isArray_5674713bb7b79043: function(arg0) {
            const ret = Array.isArray(arg0);
            return ret;
        },
        __wbg_isComposing_2ab72d608a0a8d0e: function(arg0) {
            const ret = arg0.isComposing;
            return ret;
        },
        __wbg_isConnected_03e41c8ccc9e9ad7: function(arg0) {
            const ret = arg0.isConnected;
            return ret;
        },
        __wbg_is_61443cc073056436: function(arg0, arg1) {
            const ret = Object.is(arg0, arg1);
            return ret;
        },
        __wbg_item_8ec78d4d654e9cff: function(arg0, arg1) {
            const ret = arg0.item(arg1 >>> 0);
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_item_9f80597f0653fef5: function(arg0, arg1) {
            const ret = arg0.item(arg1 >>> 0);
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_item_adf6c9a98cc42542: function(arg0, arg1) {
            const ret = arg0.item(arg1 >>> 0);
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_key_d29d670923b81510: function(arg0, arg1) {
            const ret = arg1.key;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_left_644de747afd08e57: function(arg0) {
            const ret = arg0.left;
            return ret;
        },
        __wbg_length_2a004c1681a133a1: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_length_31bdaf014f5fbde2: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_length_4e1adc0d42e23620: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_length_c5d81c9ba4a0eeed: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_length_df7d89a0ecd00dc6: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_location_382075cf6a393ad4: function(arg0) {
            const ret = arg0.location;
            return ret;
        },
        __wbg_matches_54f2cd770ee6a6ee: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.matches(getStringFromWasm0(arg1, arg2));
            return ret;
        }, arguments); },
        __wbg_metaKey_3f9183dfd5e1b726: function(arg0) {
            const ret = arg0.metaKey;
            return ret;
        },
        __wbg_metaKey_feb71e77f26b1a4c: function(arg0) {
            const ret = arg0.metaKey;
            return ret;
        },
        __wbg_name_18d155d853ac8ed4: function(arg0, arg1) {
            const ret = arg1.name;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_new_3bd534d16cd51ab5: function() { return handleError(function () {
            const ret = new CSSStyleSheet();
            return ret;
        }, arguments); },
        __wbg_new_a32a1ab6c6655abe: function(arg0, arg1) {
            const ret = new Error(getStringFromWasm0(arg0, arg1));
            return ret;
        },
        __wbg_new_bebc3f4757acf305: function() {
            const ret = new Object();
            return ret;
        },
        __wbg_new_typed_6f8b0d724fe26c07: function(arg0, arg1) {
            try {
                var state0 = {a: arg0, b: arg1};
                var cb0 = (arg0, arg1) => {
                    const a = state0.a;
                    state0.a = 0;
                    try {
                        return wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke___js_sys_6b0e3593958c45ce___Function_fn_wasm_bindgen_e306c4b1a539938c___JsValue_____wasm_bindgen_e306c4b1a539938c___sys__Undefined___js_sys_6b0e3593958c45ce___Function_fn_wasm_bindgen_e306c4b1a539938c___JsValue_____wasm_bindgen_e306c4b1a539938c___sys__Undefined_______true_(a, state0.b, arg0, arg1);
                    } finally {
                        state0.a = a;
                    }
                };
                const ret = new Promise(cb0);
                return ret;
            } finally {
                state0.a = 0;
            }
        },
        __wbg_new_with_base_15feba63f6973577: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = new URL(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
            return ret;
        }, arguments); },
        __wbg_new_with_byte_offset_and_length_492c969e8b5da8a4: function(arg0, arg1, arg2) {
            const ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
            return ret;
        },
        __wbg_nextElementSibling_c94ea03c7c9a6810: function(arg0) {
            const ret = arg0.nextElementSibling;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_nextSibling_e03804658fe4755b: function(arg0) {
            const ret = arg0.nextSibling;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_now_0b7736bc17fd7719: function(arg0) {
            const ret = arg0.now();
            return ret;
        },
        __wbg_observe_resize_93c9d31764171ae4: function(arg0, arg1, arg2) {
            const ret = arg0.observe_resize(arg1, arg2);
            return ret;
        },
        __wbg_offsetHeight_05dfe134815fef0d: function(arg0) {
            const ret = arg0.offsetHeight;
            return ret;
        },
        __wbg_offsetWidth_5f0aa67e051e1212: function(arg0) {
            const ret = arg0.offsetWidth;
            return ret;
        },
        __wbg_origin_1fc88b6b09d5c757: function(arg0, arg1) {
            const ret = arg1.origin;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_origin_68920816a16322f8: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.origin;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_parentNode_bf0725ead93471cf: function(arg0) {
            const ret = arg0.parentNode;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_pathname_184f418674522240: function(arg0, arg1) {
            const ret = arg1.pathname;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_pathname_1c74e1a4da38e765: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.pathname;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_performance_d96ad0b441f4510a: function(arg0) {
            const ret = arg0.performance;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_pointerId_b8c496f0172b1473: function(arg0) {
            const ret = arg0.pointerId;
            return ret;
        },
        __wbg_preventDefault_4235a4ccf8540533: function(arg0) {
            arg0.preventDefault();
        },
        __wbg_protocol_b85f5ff9888133f8: function(arg0) {
            const ret = arg0.protocol;
            return ret;
        },
        __wbg_pushState_1eded2f6852c83e7: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
            arg0.pushState(arg1, getStringFromWasm0(arg2, arg3), arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
        }, arguments); },
        __wbg_push_bfdf956ba476f65b: function(arg0, arg1) {
            const ret = arg0.push(arg1);
            return ret;
        },
        __wbg_querySelectorAll_7be5421ca213afe3: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.querySelectorAll(getStringFromWasm0(arg1, arg2));
            return ret;
        }, arguments); },
        __wbg_querySelectorAll_fdbf93e11103921d: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.querySelectorAll(getStringFromWasm0(arg1, arg2));
            return ret;
        }, arguments); },
        __wbg_querySelector_e621a716a5ee2aa9: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.querySelector(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_queueMicrotask_85c90f6987555d65: function(arg0) {
            const ret = arg0.queueMicrotask;
            return ret;
        },
        __wbg_queueMicrotask_f6a1fa10b81d1fc0: function(arg0) {
            queueMicrotask(arg0);
        },
        __wbg_removeAttribute_8ce0ab2fa42b383d: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.removeAttribute(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_removeChild_2d5088cc0caca88a: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.removeChild(arg1);
            return ret;
        }, arguments); },
        __wbg_removeEventListener_4c9706814dcfaf79: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.removeEventListener(getStringFromWasm0(arg1, arg2), arg3);
        }, arguments); },
        __wbg_removeEventListener_ce112b80f91bac74: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.removeEventListener(getStringFromWasm0(arg1, arg2), arg3, arg4 !== 0);
        }, arguments); },
        __wbg_remove_10865d1ab6497abe: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.remove(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_remove_2d9ed324f8c6126a: function(arg0) {
            arg0.remove();
        },
        __wbg_remove_b5cc270e088074bd: function(arg0) {
            arg0.remove();
        },
        __wbg_replaceState_3e6a7cc17847b203: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.replaceState(arg1, getStringFromWasm0(arg2, arg3));
        }, arguments); },
        __wbg_resolve_35ec7e0c6af4c82c: function(arg0) {
            const ret = Promise.resolve(arg0);
            return ret;
        },
        __wbg_respond_83a71686e927ca32: function() { return handleError(function (arg0, arg1) {
            arg0.respond(arg1 >>> 0);
        }, arguments); },
        __wbg_scrollLeft_5bebc8ba042c2371: function(arg0) {
            const ret = arg0.scrollLeft;
            return ret;
        },
        __wbg_scrollTo_d5c5bef8e15ab70b: function(arg0, arg1, arg2) {
            arg0.scrollTo(arg1, arg2);
        },
        __wbg_scrollTop_77a6c2418331d800: function(arg0) {
            const ret = arg0.scrollTop;
            return ret;
        },
        __wbg_search_685c8e05aa5dd9ba: function() { return handleError(function (arg0, arg1) {
            const ret = arg1.search;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_search_6b8de53571c80faa: function(arg0, arg1) {
            const ret = arg1.search;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_setAttribute_8c84e9351986b1f6: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_setPointerCapture_b8446eaac3edc11e: function() { return handleError(function (arg0, arg1) {
            arg0.setPointerCapture(arg1);
        }, arguments); },
        __wbg_setProperty_6139dd07828b5817: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_set_5f2ad37e5e02dc7b: function(arg0, arg1, arg2) {
            arg0.set(getArrayU8FromWasm0(arg1, arg2));
        },
        __wbg_set_a377297433dfea63: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = Reflect.set(arg0, arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_set_adoptedStyleSheets_ab2161b16edce20c: function(arg0, arg1) {
            arg0.adoptedStyleSheets = arg1;
        },
        __wbg_set_capture_db02e6717260bbc5: function(arg0, arg1) {
            arg0.capture = arg1 !== 0;
        },
        __wbg_set_checked_d4ef002a3dc5d2e4: function(arg0, arg1) {
            arg0.checked = arg1 !== 0;
        },
        __wbg_set_cssText_784c0bbb6ce18e83: function(arg0, arg1, arg2) {
            arg0.cssText = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_innerHTML_ffa3d7ec0128d447: function(arg0, arg1, arg2) {
            arg0.innerHTML = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_mode_cca1778715f365c0: function(arg0, arg1) {
            arg0.mode = __wbindgen_enum_ShadowRootMode[arg1];
        },
        __wbg_set_nodeValue_f17ed62bccdf2d9d: function(arg0, arg1, arg2) {
            arg0.nodeValue = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_once_60a969fcdc22b373: function(arg0, arg1) {
            arg0.once = arg1 !== 0;
        },
        __wbg_set_passive_1ae62aebc6856e7c: function(arg0, arg1) {
            arg0.passive = arg1 !== 0;
        },
        __wbg_set_prevent_scroll_046a03f15b24923e: function(arg0, arg1) {
            arg0.preventScroll = arg1 !== 0;
        },
        __wbg_set_signal_eb13d0f3736400da: function(arg0, arg1) {
            arg0.signal = arg1;
        },
        __wbg_set_value_23f51d0287ae7cb7: function(arg0, arg1, arg2) {
            arg0.value = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_value_6eb2c83ae62c950f: function(arg0, arg1, arg2) {
            arg0.value = getStringFromWasm0(arg1, arg2);
        },
        __wbg_shiftKey_2b651c3e5a9c07f5: function(arg0) {
            const ret = arg0.shiftKey;
            return ret;
        },
        __wbg_shiftKey_9dd20e0fb9da6eb5: function(arg0) {
            const ret = arg0.shiftKey;
            return ret;
        },
        __wbg_signal_29e36ed8ef2a125f: function(arg0) {
            const ret = arg0.signal;
            return ret;
        },
        __wbg_slice_4cddf48d47c56e19: function(arg0, arg1, arg2) {
            const ret = arg0.slice(arg1 >>> 0, arg2 >>> 0);
            return ret;
        },
        __wbg_state_a31aaa3b93bc1c7b: function() { return handleError(function (arg0) {
            const ret = arg0.state;
            return ret;
        }, arguments); },
        __wbg_static_accessor_GLOBAL_8eb4cd83130a11a0: function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_GLOBAL_THIS_1e7044f654e934db: function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_SELF_d8b50611246a6d92: function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_WINDOW_fd0bc376bf0f8b42: function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor___INCOMPLETE_CHUNKS_008a735b586f18c2: function() {
            const ret = __INCOMPLETE_CHUNKS;
            return ret;
        },
        __wbg_static_accessor___RESOLVED_RESOURCES_4670704afc0e00a2: function() {
            const ret = __RESOLVED_RESOURCES;
            return ret;
        },
        __wbg_static_accessor___SERIALIZED_ERRORS_8c0ce895061021eb: function() {
            const ret = __SERIALIZED_ERRORS;
            return ret;
        },
        __wbg_stopPropagation_39aa52c205e873cd: function(arg0) {
            arg0.stopPropagation();
        },
        __wbg_style_33dffe0e931c6262: function(arg0) {
            const ret = arg0.style;
            return ret;
        },
        __wbg_style_b79c69cee0c004b9: function(arg0) {
            const ret = arg0.style;
            return ret;
        },
        __wbg_tagName_f764922f7d763584: function(arg0, arg1) {
            const ret = arg1.tagName;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_target_3e80dc66a5936e87: function(arg0) {
            const ret = arg0.target;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_then_b830475380919203: function(arg0, arg1) {
            const ret = arg0.then(arg1);
            return ret;
        },
        __wbg_top_319e8c7a40daacf1: function(arg0) {
            const ret = arg0.top;
            return ret;
        },
        __wbg_type_e9e1e62b9a2c4682: function(arg0, arg1) {
            const ret = arg1.type;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_value_000080a4486e3ed3: function(arg0, arg1) {
            const ret = arg1.value;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_value_8215ae6d5fec3d34: function(arg0, arg1) {
            const ret = arg1.value;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_view_d8c7b26e4d4650f1: function(arg0) {
            const ret = arg0.view;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_width_71b671f940e3eb8c: function(arg0) {
            const ret = arg0.width;
            return ret;
        },
        __wbindgen_generic_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 2170, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke___wasm_bindgen_e306c4b1a539938c___JsValue______true_);
            return ret;
        },
        __wbindgen_generic_0000000000000002: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 2220, ret: Result(Unit), inner_ret: Some(Result(Unit)) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke___wasm_bindgen_e306c4b1a539938c___JsValue__core_608f92abc48d28da___result__Result_____wasm_bindgen_e306c4b1a539938c___JsError___true_);
            return ret;
        },
        __wbindgen_generic_0000000000000003: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("Event")], shim_idx: 2165, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke___web_sys_5f1bb15427655ee1___features__gen_CloseEvent__CloseEvent______true_);
            return ret;
        },
        __wbindgen_generic_0000000000000004: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("Event")], shim_idx: 2170, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke___wasm_bindgen_e306c4b1a539938c___JsValue______true__18);
            return ret;
        },
        __wbindgen_generic_0000000000000005: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("MouseEvent")], shim_idx: 2037, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke___web_sys_5f1bb15427655ee1___features__gen_MouseEvent__MouseEvent______true_);
            return ret;
        },
        __wbindgen_generic_0000000000000006: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [], shim_idx: 2167, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke_______true_);
            return ret;
        },
        __wbindgen_generic_0000000000000007: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [], shim_idx: 2173, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke_______true__1_);
            return ret;
        },
        __wbindgen_generic_0000000000000008: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return ret;
        },
        __wbindgen_generic_0000000000000009: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./app_bg.js": import0,
    };
}

function wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke_______true_(arg0, arg1) {
    wasm.wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke_______true_(arg0, arg1);
}

function wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke_______true__1_(arg0, arg1) {
    wasm.wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke_______true__1_(arg0, arg1);
}

function wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke___wasm_bindgen_e306c4b1a539938c___JsValue______true_(arg0, arg1, arg2) {
    wasm.wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke___wasm_bindgen_e306c4b1a539938c___JsValue______true_(arg0, arg1, arg2);
}

function wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke___web_sys_5f1bb15427655ee1___features__gen_CloseEvent__CloseEvent______true_(arg0, arg1, arg2) {
    wasm.wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke___web_sys_5f1bb15427655ee1___features__gen_CloseEvent__CloseEvent______true_(arg0, arg1, arg2);
}

function wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke___wasm_bindgen_e306c4b1a539938c___JsValue______true__18(arg0, arg1, arg2) {
    wasm.wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke___wasm_bindgen_e306c4b1a539938c___JsValue______true__18(arg0, arg1, arg2);
}

function wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke___web_sys_5f1bb15427655ee1___features__gen_MouseEvent__MouseEvent______true_(arg0, arg1, arg2) {
    wasm.wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke___web_sys_5f1bb15427655ee1___features__gen_MouseEvent__MouseEvent______true_(arg0, arg1, arg2);
}

function wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke___wasm_bindgen_e306c4b1a539938c___JsValue__core_608f92abc48d28da___result__Result_____wasm_bindgen_e306c4b1a539938c___JsError___true_(arg0, arg1, arg2) {
    const ret = wasm.wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke___wasm_bindgen_e306c4b1a539938c___JsValue__core_608f92abc48d28da___result__Result_____wasm_bindgen_e306c4b1a539938c___JsError___true_(arg0, arg1, arg2);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

function wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke___js_sys_6b0e3593958c45ce___Function_fn_wasm_bindgen_e306c4b1a539938c___JsValue_____wasm_bindgen_e306c4b1a539938c___sys__Undefined___js_sys_6b0e3593958c45ce___Function_fn_wasm_bindgen_e306c4b1a539938c___JsValue_____wasm_bindgen_e306c4b1a539938c___sys__Undefined_______true_(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen_e306c4b1a539938c___convert__closures_____invoke___js_sys_6b0e3593958c45ce___Function_fn_wasm_bindgen_e306c4b1a539938c___JsValue_____wasm_bindgen_e306c4b1a539938c___sys__Undefined___js_sys_6b0e3593958c45ce___Function_fn_wasm_bindgen_e306c4b1a539938c___JsValue_____wasm_bindgen_e306c4b1a539938c___sys__Undefined_______true_(arg0, arg1, arg2, arg3);
}


const __wbindgen_enum_ReadableStreamType = ["bytes"];


const __wbindgen_enum_ShadowRootMode = ["open", "closed"];
const IntoUnderlyingByteSourceFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_intounderlyingbytesource_free(ptr, 1));
const IntoUnderlyingSinkFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_intounderlyingsink_free(ptr, 1));
const IntoUnderlyingSourceFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_intounderlyingsource_free(ptr, 1));

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => wasm.__wbindgen_destroy_closure(state.a, state.b));

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeMutClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            wasm.__wbindgen_destroy_closure(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (!module.ok) {
            throw new Error(`failed to fetch Wasm: ${module.status} ${module.statusText} fetching '${module.url}'`);
        }

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }


    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
