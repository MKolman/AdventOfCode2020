
let wasm;

let WASM_VECTOR_LEN = 0;

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

let cachedTextEncoder = new TextEncoder('utf-8');

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

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
/**
* @param {string} input
* @returns {string}
*/
export function day03_part_one(input) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day03_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} input
* @returns {string}
*/
export function day03_part_two(input) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day03_part_two(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} input
* @returns {string}
*/
export function day01_part_one(input) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day01_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} input
* @returns {string}
*/
export function day01_part_two(input) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day01_part_two(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} input
* @returns {string}
*/
export function day02_part_one(input) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day02_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} input
* @returns {string}
*/
export function day02_part_two(input) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day02_part_two(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day14_part_one(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day14_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day14_part_two(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day14_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day08_part_one(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day08_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day08_part_two(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day08_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day11_part_one(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day08_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day11_part_two(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day08_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day16_part_one(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day08_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day16_part_two(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day08_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day22_part_one(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day08_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day22_part_two(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day08_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day07_part_one(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day0325_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day07_part_two(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day0325_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day10_part_one(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day0325_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day10_part_two(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day0325_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day12_part_one(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day0325_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day12_part_two(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day0325_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day18_part_one(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day0325_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day18_part_two(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day0325_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day19_part_one(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day0325_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day19_part_two(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day0325_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day21_part_one(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day0325_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day21_part_two(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day0325_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day23_part_one(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day0325_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day23_part_two(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day0325_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day0325_part_one(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day0325_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day25_part_two(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day0325_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} input
* @returns {string}
*/
export function day06_part_one(input) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day06_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} input
* @returns {string}
*/
export function day06_part_two(input) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day06_part_two(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day09_part_one(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day09_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day09_part_two(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day09_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day13_part_one(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day09_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day13_part_two(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day09_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day15_part_one(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day09_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day15_part_two(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day09_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day17_part_one(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day09_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day17_part_two(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day09_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day20_part_one(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day09_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day20_part_two(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day09_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day24_part_one(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day09_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} arg0
* @returns {string}
*/
export function day24_part_two(arg0) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day09_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} input
* @returns {string}
*/
export function day04_part_one(input) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day04_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} input
* @returns {string}
*/
export function day04_part_two(input) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day04_part_two(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} input
* @returns {string}
*/
export function day05_part_one(input) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day05_part_one(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
    }
}

/**
* @param {string} input
* @returns {string}
*/
export function day05_part_two(input) {
    try {
        const retptr = wasm.__wbindgen_export_0.value - 16;
        wasm.__wbindgen_export_0.value = retptr;
        var ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.day05_part_two(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_export_0.value += 16;
        wasm.__wbindgen_free(r0, r1);
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

async function init(input) {
    if (typeof input === 'undefined') {
        input = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    const imports = {};


    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

