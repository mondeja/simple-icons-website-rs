import { download_pdf_ } from './snippets/simple-icons-website-components-a5f0e9f58fd5accc/src/controls/download/pdf.js';
import { build_searcher, search } from './snippets/simple-icons-website-components-a5f0e9f58fd5accc/src/controls/search/fuzzy.js';

let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

const cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedFloat64Memory0 = null;

function getFloat64Memory0() {
    if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
        cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64Memory0;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

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
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

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
    if (builtInMatches.length > 1) {
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

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
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
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_36(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__ha2701ee682c86678(arg0, arg1);
}

function __wbg_adapter_39(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h111709b0ec3187ec(arg0, arg1, addHeapObject(arg2));
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b);
                state.a = 0;

            }
        }
    };
    real.original = state;

    return real;
}

let cachedUint32Memory0 = null;

function getUint32Memory0() {
    if (cachedUint32Memory0 === null || cachedUint32Memory0.byteLength === 0) {
        cachedUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32Memory0;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4);
    const mem = getUint32Memory0();
    for (let i = 0; i < array.length; i++) {
        mem[ptr / 4 + i] = addHeapObject(array[i]);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}
function __wbg_adapter_44(arg0, arg1, arg2, arg3) {
    const ptr0 = passArrayJsValueToWasm0(arg2, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm._dyn_core__ops__function__Fn__A_B___Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hbf154836e08c48fb(arg0, arg1, ptr0, len0, addHeapObject(arg3));
}

function __wbg_adapter_47(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h3f95c444a48805f2(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_50(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hc8b41f71c6ff309a(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_53(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hd8f47aac0247a62b(arg0, arg1, addHeapObject(arg2));
}

function getCachedStringFromWasm0(ptr, len) {
    if (ptr === 0) {
        return getObject(len);
    } else {
        return getStringFromWasm0(ptr, len);
    }
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
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
}

function getImports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_abda76e883ba8a5f = function() {
        const ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_658279fe44541cf6 = function(arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_error_f851667af71bcfc6 = function(arg0, arg1) {
        var v0 = getCachedStringFromWasm0(arg0, arg1);
    if (arg0 !== 0) { wasm.__wbindgen_free(arg0, arg1); }
    console.error(v0);
};
imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_cb_drop = function(arg0) {
    const obj = takeObject(arg0).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    const ret = false;
    return ret;
};
imports.wbg.__wbindgen_number_new = function(arg0) {
    const ret = arg0;
    return addHeapObject(ret);
};
imports.wbg.__wbg_buildsearcher_258c24d19e8a8eaf = function(arg0) {
    build_searcher(getObject(arg0));
};
imports.wbg.__wbg_search_ca3dc878e5d6a2e6 = function(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = search(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
    const obj = getObject(arg1);
    const ret = typeof(obj) === 'number' ? obj : undefined;
    getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
    getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
};
imports.wbg.__wbg_downloadpdf_a853e0c653f58523 = function(arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    var v1 = getCachedStringFromWasm0(arg2, arg3);
if (arg2 !== 0) { wasm.__wbindgen_free(arg2, arg3); }
download_pdf_(v0, v1);
};
imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
    const obj = getObject(arg1);
    const ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbindgen_is_string = function(arg0) {
    const ret = typeof(getObject(arg0)) === 'string';
    return ret;
};
imports.wbg.__wbindgen_is_undefined = function(arg0) {
    const ret = getObject(arg0) === undefined;
    return ret;
};
imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
    const ret = getObject(arg0) === getObject(arg1);
    return ret;
};
imports.wbg.__wbindgen_boolean_get = function(arg0) {
    const v = getObject(arg0);
    const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
    return ret;
};
imports.wbg.__wbindgen_is_null = function(arg0) {
    const ret = getObject(arg0) === null;
    return ret;
};
imports.wbg.__wbindgen_is_falsy = function(arg0) {
    const ret = !getObject(arg0);
    return ret;
};
imports.wbg.__wbg_instanceof_Window_e266f02eee43b570 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Window;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_document_950215a728589a2d = function(arg0) {
    const ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_location_797a1856892cc2de = function(arg0) {
    const ret = getObject(arg0).location;
    return addHeapObject(ret);
};
imports.wbg.__wbg_history_6ee959556f01f7ea = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).history;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_navigator_b18e629f7f0b75fa = function(arg0) {
    const ret = getObject(arg0).navigator;
    return addHeapObject(ret);
};
imports.wbg.__wbg_localStorage_42608208af988a02 = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).localStorage;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_matchMedia_967e50e4289050fa = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).matchMedia(v0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_scrollTo_881c41fdbf5d1cdd = function(arg0, arg1, arg2) {
    getObject(arg0).scrollTo(arg1, arg2);
};
imports.wbg.__wbg_fetch_465e8cb61a0f43ea = function(arg0, arg1) {
    const ret = getObject(arg0).fetch(getObject(arg1));
    return addHeapObject(ret);
};
imports.wbg.__wbg_setTimeout_6609c9aa64f32bfc = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).setTimeout(getObject(arg1), arg2);
    return ret;
}, arguments) };
imports.wbg.__wbg_documentElement_4f86b41ca49026c3 = function(arg0) {
    const ret = getObject(arg0).documentElement;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_body_be46234bb33edd63 = function(arg0) {
    const ret = getObject(arg0).body;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_head_a7ca4abdb52f6a3f = function(arg0) {
    const ret = getObject(arg0).head;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_createComment_2b861fdef485248f = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).createComment(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_createDocumentFragment_f4eafb8014772cae = function(arg0) {
    const ret = getObject(arg0).createDocumentFragment();
    return addHeapObject(ret);
};
imports.wbg.__wbg_createElement_e2a0e21263eb5416 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).createElement(v0);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_createElementNS_0047de728927ea00 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    const ret = getObject(arg0).createElementNS(v0, v1);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_createTextNode_866e33a51b47f04c = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).createTextNode(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_createTreeWalker_01d86eb08d6e8d47 = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).createTreeWalker(getObject(arg1), arg2 >>> 0);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_getElementById_eb93a47327bb5585 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).getElementById(v0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_getElementsByTagName_2c47e6868cc53cf3 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).getElementsByTagName(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_querySelector_32b9d7ebb2df951d = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).querySelector(v0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_pushState_429f091d389407b4 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    var v1 = getCachedStringFromWasm0(arg4, arg5);
    getObject(arg0).pushState(getObject(arg1), v0, v1);
}, arguments) };
imports.wbg.__wbg_replaceState_099eb1e3de220ab7 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    var v1 = getCachedStringFromWasm0(arg4, arg5);
    getObject(arg0).replaceState(getObject(arg1), v0, v1);
}, arguments) };
imports.wbg.__wbg_instanceof_HtmlInputElement_5c9d54338207f061 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof HTMLInputElement;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_value_1f2c9e357d18d3ea = function(arg0, arg1) {
    const ret = getObject(arg1).value;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_setvalue_a706abe70dab1b65 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).value = v0;
};
imports.wbg.__wbg_select_b3273cb1a48b5624 = function(arg0) {
    getObject(arg0).select();
};
imports.wbg.__wbg_length_c54fcfc679a5bfbd = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
};
imports.wbg.__wbg_newwithstrandinit_c45f0dc6da26fd03 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new Request(v0, getObject(arg2));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_getItem_f0d43fc4e780b652 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    const ret = getObject(arg1).getItem(v0);
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
}, arguments) };
imports.wbg.__wbg_setItem_f645824d6eface62 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    getObject(arg0).setItem(v0, v1);
}, arguments) };
imports.wbg.__wbg_instanceof_HtmlImageElement_e9ebe7c5a1c31239 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof HTMLImageElement;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_add_73f794d491a0e44f = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).add(v0);
}, arguments) };
imports.wbg.__wbg_contains_63ee67d3bc5da31d = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).contains(v0);
    return ret;
};
imports.wbg.__wbg_remove_f021903057d23f5e = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).remove(v0);
}, arguments) };
imports.wbg.__wbg_remove_acbc06a365649939 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    getObject(arg0).remove(v0, v1);
}, arguments) };
imports.wbg.__wbg_item_2ab86c1e3cb70ed3 = function(arg0, arg1) {
    const ret = getObject(arg0).item(arg1 >>> 0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_getwithindex_aec05ea9d2417c75 = function(arg0, arg1) {
    const ret = getObject(arg0)[arg1 >>> 0];
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_instanceof_HtmlDocument_71392a103971acc3 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof HTMLDocument;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_execCommand_e8036b7712f6700d = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).execCommand(v0);
    return ret;
}, arguments) };
imports.wbg.__wbg_settext_01c8b400b076dca5 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).text = v0;
}, arguments) };
imports.wbg.__wbg_newwithoptions_d0b8cbb7bcf679be = function() { return handleError(function (arg0, arg1) {
    const ret = new IntersectionObserver(getObject(arg0), getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_observe_6cba8679973bf84a = function(arg0, arg1) {
    getObject(arg0).observe(getObject(arg1));
};
imports.wbg.__wbg_isIntersecting_3f4c21f625385419 = function(arg0) {
    const ret = getObject(arg0).isIntersecting;
    return ret;
};
imports.wbg.__wbg_clipboard_72ae7717d527a067 = function(arg0) {
    const ret = getObject(arg0).clipboard;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_languages_6d6164e784b9ac11 = function(arg0) {
    const ret = getObject(arg0).languages;
    return addHeapObject(ret);
};
imports.wbg.__wbg_writeText_eb90459f8e0f70d4 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).writeText(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_append_1c82a9a28ac2b2c3 = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).append(getObject(arg1));
}, arguments) };
imports.wbg.__wbg_ctrlKey_e1b8f1de1eb24d5d = function(arg0) {
    const ret = getObject(arg0).ctrlKey;
    return ret;
};
imports.wbg.__wbg_shiftKey_fdd99b6df96e25c5 = function(arg0) {
    const ret = getObject(arg0).shiftKey;
    return ret;
};
imports.wbg.__wbg_altKey_d531a4d3704557cb = function(arg0) {
    const ret = getObject(arg0).altKey;
    return ret;
};
imports.wbg.__wbg_metaKey_934772989e28020c = function(arg0) {
    const ret = getObject(arg0).metaKey;
    return ret;
};
imports.wbg.__wbg_button_a1c470d5e4c997f2 = function(arg0) {
    const ret = getObject(arg0).button;
    return ret;
};
imports.wbg.__wbg_instanceof_Node_abf5312af68f179e = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Node;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_parentNode_e81e6d5dc2fc35b0 = function(arg0) {
    const ret = getObject(arg0).parentNode;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_parentElement_0e8c9afce5cb9d6e = function(arg0) {
    const ret = getObject(arg0).parentElement;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_childNodes_40585508577e98df = function(arg0) {
    const ret = getObject(arg0).childNodes;
    return addHeapObject(ret);
};
imports.wbg.__wbg_previousSibling_14035f649df13849 = function(arg0) {
    const ret = getObject(arg0).previousSibling;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_nextSibling_653f43ab9380175f = function(arg0) {
    const ret = getObject(arg0).nextSibling;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_textContent_dff59ad5e030bb86 = function(arg0, arg1) {
    const ret = getObject(arg1).textContent;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_settextContent_19dc6a6146112f16 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).textContent = v0;
};
imports.wbg.__wbg_appendChild_b8199dc1655c852d = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).appendChild(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_cloneNode_0510297160b598be = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).cloneNode();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_removeChild_794db72cbb6f21d3 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).removeChild(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_target_b629c177f9bee3da = function(arg0) {
    const ret = getObject(arg0).target;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_defaultPrevented_8ee024c3db44137d = function(arg0) {
    const ret = getObject(arg0).defaultPrevented;
    return ret;
};
imports.wbg.__wbg_cancelBubble_c9a8182589205d54 = function(arg0) {
    const ret = getObject(arg0).cancelBubble;
    return ret;
};
imports.wbg.__wbg_newwitheventinitdict_a891cfbde00e6aae = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new Event(v0, getObject(arg2));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_composedPath_d4428cc409ddd3e6 = function(arg0) {
    const ret = getObject(arg0).composedPath();
    return addHeapObject(ret);
};
imports.wbg.__wbg_preventDefault_16b2170b12f56317 = function(arg0) {
    getObject(arg0).preventDefault();
};
imports.wbg.__wbg_instanceof_HtmlHtmlElement_37d073127cfc04e9 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof HTMLHtmlElement;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_matches_7b5ad9e6bb56f1f3 = function(arg0) {
    const ret = getObject(arg0).matches;
    return ret;
};
imports.wbg.__wbg_instanceof_HtmlAnchorElement_25f125b2238e6833 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof HTMLAnchorElement;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_target_ab7ed288821255ba = function(arg0, arg1) {
    const ret = getObject(arg1).target;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_href_cbb7cd79fd1f6db8 = function(arg0, arg1) {
    const ret = getObject(arg1).href;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_instanceof_HtmlButtonElement_7046caffb25a7bfb = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof HTMLButtonElement;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_origin_4361a18d29b70c77 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg1).origin;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_pathname_7b2f7ba43a0fdd6e = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg1).pathname;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_search_23418f9752ba7ba6 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg1).search;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_hash_03f283be75af7a56 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg1).hash;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };
imports.wbg.__wbg_instanceof_Response_fb3a4df648c1859b = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Response;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_ok_1cd4c5ee1ccf4e0f = function(arg0) {
    const ret = getObject(arg0).ok;
    return ret;
};
imports.wbg.__wbg_text_f61464d781b099f0 = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).text();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_instanceof_Element_cb847a3fc7b1b1a4 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Element;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_classList_c4ebb3813d3a2f5d = function(arg0) {
    const ret = getObject(arg0).classList;
    return addHeapObject(ret);
};
imports.wbg.__wbg_setinnerHTML_76167cda24d9b96b = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).innerHTML = v0;
};
imports.wbg.__wbg_previousElementSibling_ee6d8334f306765d = function(arg0) {
    const ret = getObject(arg0).previousElementSibling;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_nextElementSibling_8517cafce4fe4aae = function(arg0) {
    const ret = getObject(arg0).nextElementSibling;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_children_93bcc921a4904ad4 = function(arg0) {
    const ret = getObject(arg0).children;
    return addHeapObject(ret);
};
imports.wbg.__wbg_firstElementChild_fe2413a9b6762430 = function(arg0) {
    const ret = getObject(arg0).firstElementChild;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_getAttribute_2c20e00a5cd314af = function(arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    const ret = getObject(arg1).getAttribute(v0);
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};
imports.wbg.__wbg_getElementsByTagName_4d80c549f09ef9e9 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).getElementsByTagName(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_hasAttribute_8d21af81e8fc2f17 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).hasAttribute(v0);
    return ret;
};
imports.wbg.__wbg_querySelector_de6e6a6226107104 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).querySelector(v0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_removeAttribute_ad7a5bf2eed30373 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).removeAttribute(v0);
}, arguments) };
imports.wbg.__wbg_scrollIntoView_9f54d6cd0a437280 = function(arg0) {
    getObject(arg0).scrollIntoView();
};
imports.wbg.__wbg_setAttribute_79c9562d32d05e66 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    getObject(arg0).setAttribute(v0, v1);
}, arguments) };
imports.wbg.__wbg_before_44f875c751453be4 = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).before(getObject(arg1));
}, arguments) };
imports.wbg.__wbg_remove_b18bc815630b67ec = function(arg0) {
    getObject(arg0).remove();
};
imports.wbg.__wbg_addEventListener_615d4590d38da1c9 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).addEventListener(v0, getObject(arg3));
}, arguments) };
imports.wbg.__wbg_dispatchEvent_615d1ccbba577081 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).dispatchEvent(getObject(arg1));
    return ret;
}, arguments) };
imports.wbg.__wbg_origin_79c420abc5f23f9f = function(arg0, arg1) {
    const ret = getObject(arg1).origin;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_pathname_188be9b0ca3ddf22 = function(arg0, arg1) {
    const ret = getObject(arg1).pathname;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_search_8b081e18a33be3ec = function(arg0, arg1) {
    const ret = getObject(arg1).search;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_searchParams_da95a791f35624b6 = function(arg0) {
    const ret = getObject(arg0).searchParams;
    return addHeapObject(ret);
};
imports.wbg.__wbg_hash_5f41a2c79884b4e0 = function(arg0, arg1) {
    const ret = getObject(arg1).hash;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_newwithbase_5541ad423d71a320 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    var v1 = getCachedStringFromWasm0(arg2, arg3);
    const ret = new URL(v0, v1);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_setdata_c26e68878a7d634c = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).data = v0;
};
imports.wbg.__wbg_before_e11d7f9398d1334d = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).before(getObject(arg1));
}, arguments) };
imports.wbg.__wbg_remove_f2f10b21db578dab = function(arg0) {
    getObject(arg0).remove();
};
imports.wbg.__wbg_instanceof_HtmlElement_9e442d53bb553421 = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof HTMLElement;
    } catch {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_setlang_c0fbc6a54b23270a = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).lang = v0;
};
imports.wbg.__wbg_innerText_48abe0676a97aa32 = function(arg0, arg1) {
    const ret = getObject(arg1).innerText;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_setinnerText_3dfcf413b5b5621c = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).innerText = v0;
};
imports.wbg.__wbg_blur_b94a30ed202c3612 = function() { return handleError(function (arg0) {
    getObject(arg0).blur();
}, arguments) };
imports.wbg.__wbg_click_f3e37a4a2595187d = function(arg0) {
    getObject(arg0).click();
};
imports.wbg.__wbg_focus_6497e1b44dabfb24 = function() { return handleError(function (arg0) {
    getObject(arg0).focus();
}, arguments) };
imports.wbg.__wbg_nextNode_8b656aba7fde94a8 = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).nextNode();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_debug_8db2eed1bf6c1e2a = function(arg0) {
    console.debug(getObject(arg0));
};
imports.wbg.__wbg_error_fe807da27c4a4ced = function(arg0) {
    console.error(getObject(arg0));
};
imports.wbg.__wbg_info_9e6db45ac337c3b5 = function(arg0) {
    console.info(getObject(arg0));
};
imports.wbg.__wbg_log_7bb108d119bafbc1 = function(arg0) {
    console.log(getObject(arg0));
};
imports.wbg.__wbg_warn_e57696dbb3977030 = function(arg0) {
    console.warn(getObject(arg0));
};
imports.wbg.__wbg_get_27fe3dac1c4d0224 = function(arg0, arg1) {
    const ret = getObject(arg0)[arg1 >>> 0];
    return addHeapObject(ret);
};
imports.wbg.__wbg_length_e498fbc24f9c1d4f = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
};
imports.wbg.__wbg_new_b525de17f44a8943 = function() {
    const ret = new Array();
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_is_function = function(arg0) {
    const ret = typeof(getObject(arg0)) === 'function';
    return ret;
};
imports.wbg.__wbg_newnoargs_2b8b6bd7753c76ba = function(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new Function(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_is_object = function(arg0) {
    const val = getObject(arg0);
    const ret = typeof(val) === 'object' && val !== null;
    return ret;
};
imports.wbg.__wbg_next_b7d530c04fd8b217 = function(arg0) {
    const ret = getObject(arg0).next;
    return addHeapObject(ret);
};
imports.wbg.__wbg_next_88560ec06a094dea = function() { return handleError(function (arg0) {
    const ret = getObject(arg0).next();
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_done_1ebec03bbd919843 = function(arg0) {
    const ret = getObject(arg0).done;
    return ret;
};
imports.wbg.__wbg_value_6ac8da5cc5b3efda = function(arg0) {
    const ret = getObject(arg0).value;
    return addHeapObject(ret);
};
imports.wbg.__wbg_iterator_55f114446221aa5a = function() {
    const ret = Symbol.iterator;
    return addHeapObject(ret);
};
imports.wbg.__wbg_get_baf4855f9a986186 = function() { return handleError(function (arg0, arg1) {
    const ret = Reflect.get(getObject(arg0), getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_call_95d1ea488d03e4e8 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_new_f9876326328f45ed = function() {
    const ret = new Object();
    return addHeapObject(ret);
};
imports.wbg.__wbg_self_e7c1f827057f6584 = function() { return handleError(function () {
    const ret = self.self;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_window_a09ec664e14b1b81 = function() { return handleError(function () {
    const ret = window.window;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_globalThis_87cbb8506fecf3a9 = function() { return handleError(function () {
    const ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_global_c85a9259e621f3db = function() { return handleError(function () {
    const ret = global.global;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_decodeURI_6912fcb8d05c69eb = function() { return handleError(function (arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = decodeURI(v0);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_encodeURI_be0018f463b5bef8 = function(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = encodeURI(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_from_67ca20fa722467e6 = function(arg0) {
    const ret = Array.from(getObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_isArray_39d28997bf6b96b4 = function(arg0) {
    const ret = Array.isArray(getObject(arg0));
    return ret;
};
imports.wbg.__wbg_of_defbc28e6c20f48f = function(arg0, arg1) {
    const ret = Array.of(getObject(arg0), getObject(arg1));
    return addHeapObject(ret);
};
imports.wbg.__wbg_push_49c286f04dd3bf59 = function(arg0, arg1) {
    const ret = getObject(arg0).push(getObject(arg1));
    return ret;
};
imports.wbg.__wbg_call_9495de66fdbe016b = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_new_f127e324c1313064 = function(arg0) {
    const ret = new Date(getObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_toLocaleDateString_1ec649bb85179998 = function(arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).toLocaleDateString(v0, getObject(arg3));
    return addHeapObject(ret);
};
imports.wbg.__wbg_is_8f1618fe9a4fd388 = function(arg0, arg1) {
    const ret = Object.is(getObject(arg0), getObject(arg1));
    return ret;
};
imports.wbg.__wbg_exec_9d6be66864fc3c35 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = getObject(arg0).exec(v0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_new_bf04732a17426e8f = function(arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    var v1 = getCachedStringFromWasm0(arg2, arg3);
    const ret = new RegExp(v0, v1);
    return addHeapObject(ret);
};
imports.wbg.__wbg_resolve_fd40f858d9db1a04 = function(arg0) {
    const ret = Promise.resolve(getObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_then_ec5db6d509eb475f = function(arg0, arg1) {
    const ret = getObject(arg0).then(getObject(arg1));
    return addHeapObject(ret);
};
imports.wbg.__wbg_then_f753623316e2873a = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
};
imports.wbg.__wbg_set_6aa458a4ebdb65cb = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
    return ret;
}, arguments) };
imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
    const ret = debugString(getObject(arg1));
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};
imports.wbg.__wbindgen_closure_wrapper606 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 119, __wbg_adapter_36);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper608 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 119, __wbg_adapter_39);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper610 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 119, __wbg_adapter_39);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper612 = function(arg0, arg1, arg2) {
    const ret = makeClosure(arg0, arg1, 119, __wbg_adapter_44);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper1683 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 596, __wbg_adapter_47);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper1929 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 670, __wbg_adapter_50);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper3696 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 856, __wbg_adapter_53);
    return addHeapObject(ret);
};

return imports;
}

function initMemory(imports, maybe_memory) {

}

function finalizeInit(instance, module) {
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    cachedFloat64Memory0 = null;
    cachedInt32Memory0 = null;
    cachedUint32Memory0 = null;
    cachedUint8Memory0 = null;

    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    const imports = getImports();

    initMemory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return finalizeInit(instance, module);
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('simple-icons-website-d5d675c076994a31_bg.wasm', import.meta.url);
    }
    const imports = getImports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    initMemory(imports);

    const { instance, module } = await load(await input, imports);

    return finalizeInit(instance, module);
}

export { initSync }
export default init;
