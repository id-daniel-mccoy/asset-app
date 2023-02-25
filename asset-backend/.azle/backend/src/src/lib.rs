#![allow(warnings, unused)]
use azle_vm_value_derive::{CdkActTryFromVmValue, CdkActTryIntoVmValue};
use ic_cdk::api::call::CallResult;
use rand::{rngs::StdRng, Rng, SeedableRng};
use slotmap::Key as AzleSlotMapKey;
use std::borrow::BorrowMut;
use std::convert::TryInto;
use std::str::FromStr;
thread_local! { static BOA_CONTEXT_REF_CELL : std :: cell :: RefCell < boa_engine :: Context > = std :: cell :: RefCell :: new (boa_engine :: Context :: default ()) ; static PROMISE_MAP_REF_CELL : std :: cell :: RefCell < std :: collections :: HashMap < String , boa_engine :: JsValue > , > = std :: cell :: RefCell :: new (std :: collections :: HashMap :: new ()) ; static UUID_REF_CELL : std :: cell :: RefCell < String > = std :: cell :: RefCell :: new ("" . to_string ()) ; static METHOD_NAME_REF_CELL : std :: cell :: RefCell < String > = std :: cell :: RefCell :: new ("" . to_string ()) ; static MANUAL_REF_CELL : std :: cell :: RefCell < bool > = std :: cell :: RefCell :: new (false) ; static RNG_REF_CELL : std :: cell :: RefCell < StdRng > = std :: cell :: RefCell :: new (SeedableRng :: from_seed ([0u8 ; 32])) ; }
static MAIN_JS : & 'static str = "\n            // TODO we should centralize/standardize where we add global variables to the JS, we are doing this in multiple places (i.e. the exports variable is not here, found in init/post_upgrade)\n            globalThis.console = {\n                ...globalThis.console,\n                log: (...args) => {\n                    ic.print(...args);\n                }\n            };\n\n            \nObject.defineProperty(exports, \"__esModule\", {\n    value: true\n});\nexports.hello_world = exports.Principal = void 0;\nvar __create = Object.create;\nvar __defProp = Object.defineProperty;\nvar __getOwnPropDesc = Object.getOwnPropertyDescriptor;\nvar __getOwnPropNames = Object.getOwnPropertyNames;\nvar __getProtoOf = Object.getPrototypeOf;\nvar __hasOwnProp = Object.prototype.hasOwnProperty;\nvar __markAsModule = (target)=>__defProp(target, \"__esModule\", {\n        value: true\n    })\n;\nvar __commonJS = (cb, mod)=>function __require() {\n        return mod || (0, cb[__getOwnPropNames(cb)[0]])((mod = {\n            exports: {}\n        }).exports, mod), mod.exports;\n    }\n;\nvar __reExport = (target, module2, copyDefault, desc)=>{\n    if (module2 && typeof module2 === \"object\" || typeof module2 === \"function\") {\n        for (let key of __getOwnPropNames(module2))if (!__hasOwnProp.call(target, key) && (copyDefault || key !== \"default\")) __defProp(target, key, {\n            get: ()=>module2[key]\n            ,\n            enumerable: !(desc = __getOwnPropDesc(module2, key)) || desc.enumerable\n        });\n    }\n    return target;\n};\nvar __toESM = (module2, isNodeMode)=>{\n    return __reExport(__markAsModule(__defProp(module2 != null ? __create(__getProtoOf(module2)) : {}, \"default\", !isNodeMode && module2 && module2.__esModule ? {\n        get: ()=>module2.default\n        ,\n        enumerable: true\n    } : {\n        value: module2,\n        enumerable: true\n    })), module2);\n};\n// node_modules/js-sha256/src/sha256.js\nvar require_sha256 = __commonJS({\n    \"node_modules/js-sha256/src/sha256.js\" (exports1, module) {\n        (function() {\n            \n            var ERROR = \"input is invalid type\";\n            var WINDOW = typeof window === \"object\";\n            var root = WINDOW ? window : {};\n            if (root.JS_SHA256_NO_WINDOW) {\n                WINDOW = false;\n            }\n            var WEB_WORKER = !WINDOW && typeof self === \"object\";\n            var NODE_JS = !root.JS_SHA256_NO_NODE_JS && typeof process === \"object\" && process.versions && process.versions.node;\n            if (NODE_JS) {\n                root = global;\n            } else if (WEB_WORKER) {\n                root = self;\n            }\n            var COMMON_JS = !root.JS_SHA256_NO_COMMON_JS && typeof module === \"object\" && module.exports;\n            var AMD = typeof define === \"function\" && define.amd;\n            var ARRAY_BUFFER = !root.JS_SHA256_NO_ARRAY_BUFFER && typeof ArrayBuffer !== \"undefined\";\n            var HEX_CHARS = \"0123456789abcdef\".split(\"\");\n            var EXTRA = [\n                -2147483648,\n                8388608,\n                32768,\n                128\n            ];\n            var SHIFT = [\n                24,\n                16,\n                8,\n                0\n            ];\n            var K = [\n                1116352408,\n                1899447441,\n                3049323471,\n                3921009573,\n                961987163,\n                1508970993,\n                2453635748,\n                2870763221,\n                3624381080,\n                310598401,\n                607225278,\n                1426881987,\n                1925078388,\n                2162078206,\n                2614888103,\n                3248222580,\n                3835390401,\n                4022224774,\n                264347078,\n                604807628,\n                770255983,\n                1249150122,\n                1555081692,\n                1996064986,\n                2554220882,\n                2821834349,\n                2952996808,\n                3210313671,\n                3336571891,\n                3584528711,\n                113926993,\n                338241895,\n                666307205,\n                773529912,\n                1294757372,\n                1396182291,\n                1695183700,\n                1986661051,\n                2177026350,\n                2456956037,\n                2730485921,\n                2820302411,\n                3259730800,\n                3345764771,\n                3516065817,\n                3600352804,\n                4094571909,\n                275423344,\n                430227734,\n                506948616,\n                659060556,\n                883997877,\n                958139571,\n                1322822218,\n                1537002063,\n                1747873779,\n                1955562222,\n                2024104815,\n                2227730452,\n                2361852424,\n                2428436474,\n                2756734187,\n                3204031479,\n                3329325298\n            ];\n            var OUTPUT_TYPES = [\n                \"hex\",\n                \"array\",\n                \"digest\",\n                \"arrayBuffer\"\n            ];\n            var blocks = [];\n            if (root.JS_SHA256_NO_NODE_JS || !Array.isArray) {\n                Array.isArray = function(obj) {\n                    return Object.prototype.toString.call(obj) === \"[object Array]\";\n                };\n            }\n            if (ARRAY_BUFFER && (root.JS_SHA256_NO_ARRAY_BUFFER_IS_VIEW || !ArrayBuffer.isView)) {\n                ArrayBuffer.isView = function(obj) {\n                    return typeof obj === \"object\" && obj.buffer && obj.buffer.constructor === ArrayBuffer;\n                };\n            }\n            var createOutputMethod = function(outputType, is2242) {\n                return function(message) {\n                    return new Sha256(is2242, true).update(message)[outputType]();\n                };\n            };\n            var createMethod = function(is2242) {\n                var method2 = createOutputMethod(\"hex\", is2242);\n                if (NODE_JS) {\n                    method2 = nodeWrap(method2, is2242);\n                }\n                method2.create = function() {\n                    return new Sha256(is2242);\n                };\n                method2.update = function(message) {\n                    return method2.create().update(message);\n                };\n                for(var i2 = 0; i2 < OUTPUT_TYPES.length; ++i2){\n                    var type = OUTPUT_TYPES[i2];\n                    method2[type] = createOutputMethod(type, is2242);\n                }\n                return method2;\n            };\n            var nodeWrap = function(method, is224) {\n                var crypto = eval(\"require('crypto')\");\n                var Buffer = eval(\"require('buffer').Buffer\");\n                var algorithm = is224 ? \"sha224\" : \"sha256\";\n                var nodeMethod = function(message) {\n                    if (typeof message === \"string\") {\n                        return crypto.createHash(algorithm).update(message, \"utf8\").digest(\"hex\");\n                    } else {\n                        if (message === null || message === void 0) {\n                            throw new Error(ERROR);\n                        } else if (message.constructor === ArrayBuffer) {\n                            message = new Uint8Array(message);\n                        }\n                    }\n                    if (Array.isArray(message) || ArrayBuffer.isView(message) || message.constructor === Buffer) {\n                        return crypto.createHash(algorithm).update(new Buffer(message)).digest(\"hex\");\n                    } else {\n                        return method(message);\n                    }\n                };\n                return nodeMethod;\n            };\n            var createHmacOutputMethod = function(outputType, is2242) {\n                return function(key, message) {\n                    return new HmacSha256(key, is2242, true).update(message)[outputType]();\n                };\n            };\n            var createHmacMethod = function(is2242) {\n                var method2 = createHmacOutputMethod(\"hex\", is2242);\n                method2.create = function(key) {\n                    return new HmacSha256(key, is2242);\n                };\n                method2.update = function(key, message) {\n                    return method2.create(key).update(message);\n                };\n                for(var i3 = 0; i3 < OUTPUT_TYPES.length; ++i3){\n                    var type = OUTPUT_TYPES[i3];\n                    method2[type] = createHmacOutputMethod(type, is2242);\n                }\n                return method2;\n            };\n            function Sha256(is2242, sharedMemory) {\n                if (sharedMemory) {\n                    blocks[0] = blocks[16] = blocks[1] = blocks[2] = blocks[3] = blocks[4] = blocks[5] = blocks[6] = blocks[7] = blocks[8] = blocks[9] = blocks[10] = blocks[11] = blocks[12] = blocks[13] = blocks[14] = blocks[15] = 0;\n                    this.blocks = blocks;\n                } else {\n                    this.blocks = [\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0,\n                        0\n                    ];\n                }\n                if (is2242) {\n                    this.h0 = 3238371032;\n                    this.h1 = 914150663;\n                    this.h2 = 812702999;\n                    this.h3 = 4144912697;\n                    this.h4 = 4290775857;\n                    this.h5 = 1750603025;\n                    this.h6 = 1694076839;\n                    this.h7 = 3204075428;\n                } else {\n                    this.h0 = 1779033703;\n                    this.h1 = 3144134277;\n                    this.h2 = 1013904242;\n                    this.h3 = 2773480762;\n                    this.h4 = 1359893119;\n                    this.h5 = 2600822924;\n                    this.h6 = 528734635;\n                    this.h7 = 1541459225;\n                }\n                this.block = this.start = this.bytes = this.hBytes = 0;\n                this.finalized = this.hashed = false;\n                this.first = true;\n                this.is224 = is2242;\n            }\n            Sha256.prototype.update = function(message) {\n                if (this.finalized) {\n                    return;\n                }\n                var notString, type = typeof message;\n                if (type !== \"string\") {\n                    if (type === \"object\") {\n                        if (message === null) {\n                            throw new Error(ERROR);\n                        } else if (ARRAY_BUFFER && message.constructor === ArrayBuffer) {\n                            message = new Uint8Array(message);\n                        } else if (!Array.isArray(message)) {\n                            if (!ARRAY_BUFFER || !ArrayBuffer.isView(message)) {\n                                throw new Error(ERROR);\n                            }\n                        }\n                    } else {\n                        throw new Error(ERROR);\n                    }\n                    notString = true;\n                }\n                var code, index = 0, i4, length = message.length, blocks2 = this.blocks;\n                while(index < length){\n                    if (this.hashed) {\n                        this.hashed = false;\n                        blocks2[0] = this.block;\n                        blocks2[16] = blocks2[1] = blocks2[2] = blocks2[3] = blocks2[4] = blocks2[5] = blocks2[6] = blocks2[7] = blocks2[8] = blocks2[9] = blocks2[10] = blocks2[11] = blocks2[12] = blocks2[13] = blocks2[14] = blocks2[15] = 0;\n                    }\n                    if (notString) {\n                        for(i4 = this.start; index < length && i4 < 64; ++index){\n                            blocks2[i4 >> 2] |= message[index] << SHIFT[(i4++) & 3];\n                        }\n                    } else {\n                        for(i4 = this.start; index < length && i4 < 64; ++index){\n                            code = message.charCodeAt(index);\n                            if (code < 128) {\n                                blocks2[i4 >> 2] |= code << SHIFT[(i4++) & 3];\n                            } else if (code < 2048) {\n                                blocks2[i4 >> 2] |= (192 | code >> 6) << SHIFT[(i4++) & 3];\n                                blocks2[i4 >> 2] |= (128 | code & 63) << SHIFT[(i4++) & 3];\n                            } else if (code < 55296 || code >= 57344) {\n                                blocks2[i4 >> 2] |= (224 | code >> 12) << SHIFT[(i4++) & 3];\n                                blocks2[i4 >> 2] |= (128 | code >> 6 & 63) << SHIFT[(i4++) & 3];\n                                blocks2[i4 >> 2] |= (128 | code & 63) << SHIFT[(i4++) & 3];\n                            } else {\n                                code = 65536 + ((code & 1023) << 10 | message.charCodeAt(++index) & 1023);\n                                blocks2[i4 >> 2] |= (240 | code >> 18) << SHIFT[(i4++) & 3];\n                                blocks2[i4 >> 2] |= (128 | code >> 12 & 63) << SHIFT[(i4++) & 3];\n                                blocks2[i4 >> 2] |= (128 | code >> 6 & 63) << SHIFT[(i4++) & 3];\n                                blocks2[i4 >> 2] |= (128 | code & 63) << SHIFT[(i4++) & 3];\n                            }\n                        }\n                    }\n                    this.lastByteIndex = i4;\n                    this.bytes += i4 - this.start;\n                    if (i4 >= 64) {\n                        this.block = blocks2[16];\n                        this.start = i4 - 64;\n                        this.hash();\n                        this.hashed = true;\n                    } else {\n                        this.start = i4;\n                    }\n                }\n                if (this.bytes > 4294967295) {\n                    this.hBytes += this.bytes / 4294967296 << 0;\n                    this.bytes = this.bytes % 4294967296;\n                }\n                return this;\n            };\n            Sha256.prototype.finalize = function() {\n                if (this.finalized) {\n                    return;\n                }\n                this.finalized = true;\n                var blocks2 = this.blocks, i5 = this.lastByteIndex;\n                blocks2[16] = this.block;\n                blocks2[i5 >> 2] |= EXTRA[i5 & 3];\n                this.block = blocks2[16];\n                if (i5 >= 56) {\n                    if (!this.hashed) {\n                        this.hash();\n                    }\n                    blocks2[0] = this.block;\n                    blocks2[16] = blocks2[1] = blocks2[2] = blocks2[3] = blocks2[4] = blocks2[5] = blocks2[6] = blocks2[7] = blocks2[8] = blocks2[9] = blocks2[10] = blocks2[11] = blocks2[12] = blocks2[13] = blocks2[14] = blocks2[15] = 0;\n                }\n                blocks2[14] = this.hBytes << 3 | this.bytes >>> 29;\n                blocks2[15] = this.bytes << 3;\n                this.hash();\n            };\n            Sha256.prototype.hash = function() {\n                var a = this.h0, b = this.h1, c = this.h2, d = this.h3, e = this.h4, f = this.h5, g = this.h6, h = this.h7, blocks2 = this.blocks, j, s0, s1, maj, t1, t2, ch, ab, da, cd, bc;\n                for(j = 16; j < 64; ++j){\n                    t1 = blocks2[j - 15];\n                    s0 = (t1 >>> 7 | t1 << 25) ^ (t1 >>> 18 | t1 << 14) ^ t1 >>> 3;\n                    t1 = blocks2[j - 2];\n                    s1 = (t1 >>> 17 | t1 << 15) ^ (t1 >>> 19 | t1 << 13) ^ t1 >>> 10;\n                    blocks2[j] = blocks2[j - 16] + s0 + blocks2[j - 7] + s1 << 0;\n                }\n                bc = b & c;\n                for(j = 0; j < 64; j += 4){\n                    if (this.first) {\n                        if (this.is224) {\n                            ab = 300032;\n                            t1 = blocks2[0] - 1413257819;\n                            h = t1 - 150054599 << 0;\n                            d = t1 + 24177077 << 0;\n                        } else {\n                            ab = 704751109;\n                            t1 = blocks2[0] - 210244248;\n                            h = t1 - 1521486534 << 0;\n                            d = t1 + 143694565 << 0;\n                        }\n                        this.first = false;\n                    } else {\n                        s0 = (a >>> 2 | a << 30) ^ (a >>> 13 | a << 19) ^ (a >>> 22 | a << 10);\n                        s1 = (e >>> 6 | e << 26) ^ (e >>> 11 | e << 21) ^ (e >>> 25 | e << 7);\n                        ab = a & b;\n                        maj = ab ^ a & c ^ bc;\n                        ch = e & f ^ ~e & g;\n                        t1 = h + s1 + ch + K[j] + blocks2[j];\n                        t2 = s0 + maj;\n                        h = d + t1 << 0;\n                        d = t1 + t2 << 0;\n                    }\n                    s0 = (d >>> 2 | d << 30) ^ (d >>> 13 | d << 19) ^ (d >>> 22 | d << 10);\n                    s1 = (h >>> 6 | h << 26) ^ (h >>> 11 | h << 21) ^ (h >>> 25 | h << 7);\n                    da = d & a;\n                    maj = da ^ d & b ^ ab;\n                    ch = h & e ^ ~h & f;\n                    t1 = g + s1 + ch + K[j + 1] + blocks2[j + 1];\n                    t2 = s0 + maj;\n                    g = c + t1 << 0;\n                    c = t1 + t2 << 0;\n                    s0 = (c >>> 2 | c << 30) ^ (c >>> 13 | c << 19) ^ (c >>> 22 | c << 10);\n                    s1 = (g >>> 6 | g << 26) ^ (g >>> 11 | g << 21) ^ (g >>> 25 | g << 7);\n                    cd = c & d;\n                    maj = cd ^ c & a ^ da;\n                    ch = g & h ^ ~g & e;\n                    t1 = f + s1 + ch + K[j + 2] + blocks2[j + 2];\n                    t2 = s0 + maj;\n                    f = b + t1 << 0;\n                    b = t1 + t2 << 0;\n                    s0 = (b >>> 2 | b << 30) ^ (b >>> 13 | b << 19) ^ (b >>> 22 | b << 10);\n                    s1 = (f >>> 6 | f << 26) ^ (f >>> 11 | f << 21) ^ (f >>> 25 | f << 7);\n                    bc = b & c;\n                    maj = bc ^ b & d ^ cd;\n                    ch = f & g ^ ~f & h;\n                    t1 = e + s1 + ch + K[j + 3] + blocks2[j + 3];\n                    t2 = s0 + maj;\n                    e = a + t1 << 0;\n                    a = t1 + t2 << 0;\n                }\n                this.h0 = this.h0 + a << 0;\n                this.h1 = this.h1 + b << 0;\n                this.h2 = this.h2 + c << 0;\n                this.h3 = this.h3 + d << 0;\n                this.h4 = this.h4 + e << 0;\n                this.h5 = this.h5 + f << 0;\n                this.h6 = this.h6 + g << 0;\n                this.h7 = this.h7 + h << 0;\n            };\n            Sha256.prototype.hex = function() {\n                this.finalize();\n                var h0 = this.h0, h1 = this.h1, h2 = this.h2, h3 = this.h3, h4 = this.h4, h5 = this.h5, h6 = this.h6, h7 = this.h7;\n                var hex = HEX_CHARS[h0 >> 28 & 15] + HEX_CHARS[h0 >> 24 & 15] + HEX_CHARS[h0 >> 20 & 15] + HEX_CHARS[h0 >> 16 & 15] + HEX_CHARS[h0 >> 12 & 15] + HEX_CHARS[h0 >> 8 & 15] + HEX_CHARS[h0 >> 4 & 15] + HEX_CHARS[h0 & 15] + HEX_CHARS[h1 >> 28 & 15] + HEX_CHARS[h1 >> 24 & 15] + HEX_CHARS[h1 >> 20 & 15] + HEX_CHARS[h1 >> 16 & 15] + HEX_CHARS[h1 >> 12 & 15] + HEX_CHARS[h1 >> 8 & 15] + HEX_CHARS[h1 >> 4 & 15] + HEX_CHARS[h1 & 15] + HEX_CHARS[h2 >> 28 & 15] + HEX_CHARS[h2 >> 24 & 15] + HEX_CHARS[h2 >> 20 & 15] + HEX_CHARS[h2 >> 16 & 15] + HEX_CHARS[h2 >> 12 & 15] + HEX_CHARS[h2 >> 8 & 15] + HEX_CHARS[h2 >> 4 & 15] + HEX_CHARS[h2 & 15] + HEX_CHARS[h3 >> 28 & 15] + HEX_CHARS[h3 >> 24 & 15] + HEX_CHARS[h3 >> 20 & 15] + HEX_CHARS[h3 >> 16 & 15] + HEX_CHARS[h3 >> 12 & 15] + HEX_CHARS[h3 >> 8 & 15] + HEX_CHARS[h3 >> 4 & 15] + HEX_CHARS[h3 & 15] + HEX_CHARS[h4 >> 28 & 15] + HEX_CHARS[h4 >> 24 & 15] + HEX_CHARS[h4 >> 20 & 15] + HEX_CHARS[h4 >> 16 & 15] + HEX_CHARS[h4 >> 12 & 15] + HEX_CHARS[h4 >> 8 & 15] + HEX_CHARS[h4 >> 4 & 15] + HEX_CHARS[h4 & 15] + HEX_CHARS[h5 >> 28 & 15] + HEX_CHARS[h5 >> 24 & 15] + HEX_CHARS[h5 >> 20 & 15] + HEX_CHARS[h5 >> 16 & 15] + HEX_CHARS[h5 >> 12 & 15] + HEX_CHARS[h5 >> 8 & 15] + HEX_CHARS[h5 >> 4 & 15] + HEX_CHARS[h5 & 15] + HEX_CHARS[h6 >> 28 & 15] + HEX_CHARS[h6 >> 24 & 15] + HEX_CHARS[h6 >> 20 & 15] + HEX_CHARS[h6 >> 16 & 15] + HEX_CHARS[h6 >> 12 & 15] + HEX_CHARS[h6 >> 8 & 15] + HEX_CHARS[h6 >> 4 & 15] + HEX_CHARS[h6 & 15];\n                if (!this.is224) {\n                    hex += HEX_CHARS[h7 >> 28 & 15] + HEX_CHARS[h7 >> 24 & 15] + HEX_CHARS[h7 >> 20 & 15] + HEX_CHARS[h7 >> 16 & 15] + HEX_CHARS[h7 >> 12 & 15] + HEX_CHARS[h7 >> 8 & 15] + HEX_CHARS[h7 >> 4 & 15] + HEX_CHARS[h7 & 15];\n                }\n                return hex;\n            };\n            Sha256.prototype.toString = Sha256.prototype.hex;\n            Sha256.prototype.digest = function() {\n                this.finalize();\n                var h0 = this.h0, h1 = this.h1, h2 = this.h2, h3 = this.h3, h4 = this.h4, h5 = this.h5, h6 = this.h6, h7 = this.h7;\n                var arr = [\n                    h0 >> 24 & 255,\n                    h0 >> 16 & 255,\n                    h0 >> 8 & 255,\n                    h0 & 255,\n                    h1 >> 24 & 255,\n                    h1 >> 16 & 255,\n                    h1 >> 8 & 255,\n                    h1 & 255,\n                    h2 >> 24 & 255,\n                    h2 >> 16 & 255,\n                    h2 >> 8 & 255,\n                    h2 & 255,\n                    h3 >> 24 & 255,\n                    h3 >> 16 & 255,\n                    h3 >> 8 & 255,\n                    h3 & 255,\n                    h4 >> 24 & 255,\n                    h4 >> 16 & 255,\n                    h4 >> 8 & 255,\n                    h4 & 255,\n                    h5 >> 24 & 255,\n                    h5 >> 16 & 255,\n                    h5 >> 8 & 255,\n                    h5 & 255,\n                    h6 >> 24 & 255,\n                    h6 >> 16 & 255,\n                    h6 >> 8 & 255,\n                    h6 & 255\n                ];\n                if (!this.is224) {\n                    arr.push(h7 >> 24 & 255, h7 >> 16 & 255, h7 >> 8 & 255, h7 & 255);\n                }\n                return arr;\n            };\n            Sha256.prototype.array = Sha256.prototype.digest;\n            Sha256.prototype.arrayBuffer = function() {\n                this.finalize();\n                var buffer = new ArrayBuffer(this.is224 ? 28 : 32);\n                var dataView = new DataView(buffer);\n                dataView.setUint32(0, this.h0);\n                dataView.setUint32(4, this.h1);\n                dataView.setUint32(8, this.h2);\n                dataView.setUint32(12, this.h3);\n                dataView.setUint32(16, this.h4);\n                dataView.setUint32(20, this.h5);\n                dataView.setUint32(24, this.h6);\n                if (!this.is224) {\n                    dataView.setUint32(28, this.h7);\n                }\n                return buffer;\n            };\n            function HmacSha256(key, is2242, sharedMemory) {\n                var i6, type = typeof key;\n                if (type === \"string\") {\n                    var bytes = [], length = key.length, index = 0, code;\n                    for(i6 = 0; i6 < length; ++i6){\n                        code = key.charCodeAt(i6);\n                        if (code < 128) {\n                            bytes[index++] = code;\n                        } else if (code < 2048) {\n                            bytes[index++] = 192 | code >> 6;\n                            bytes[index++] = 128 | code & 63;\n                        } else if (code < 55296 || code >= 57344) {\n                            bytes[index++] = 224 | code >> 12;\n                            bytes[index++] = 128 | code >> 6 & 63;\n                            bytes[index++] = 128 | code & 63;\n                        } else {\n                            code = 65536 + ((code & 1023) << 10 | key.charCodeAt(++i6) & 1023);\n                            bytes[index++] = 240 | code >> 18;\n                            bytes[index++] = 128 | code >> 12 & 63;\n                            bytes[index++] = 128 | code >> 6 & 63;\n                            bytes[index++] = 128 | code & 63;\n                        }\n                    }\n                    key = bytes;\n                } else {\n                    if (type === \"object\") {\n                        if (key === null) {\n                            throw new Error(ERROR);\n                        } else if (ARRAY_BUFFER && key.constructor === ArrayBuffer) {\n                            key = new Uint8Array(key);\n                        } else if (!Array.isArray(key)) {\n                            if (!ARRAY_BUFFER || !ArrayBuffer.isView(key)) {\n                                throw new Error(ERROR);\n                            }\n                        }\n                    } else {\n                        throw new Error(ERROR);\n                    }\n                }\n                if (key.length > 64) {\n                    key = new Sha256(is2242, true).update(key).array();\n                }\n                var oKeyPad = [], iKeyPad = [];\n                for(i6 = 0; i6 < 64; ++i6){\n                    var b = key[i6] || 0;\n                    oKeyPad[i6] = 92 ^ b;\n                    iKeyPad[i6] = 54 ^ b;\n                }\n                Sha256.call(this, is2242, sharedMemory);\n                this.update(iKeyPad);\n                this.oKeyPad = oKeyPad;\n                this.inner = true;\n                this.sharedMemory = sharedMemory;\n            }\n            HmacSha256.prototype = new Sha256();\n            HmacSha256.prototype.finalize = function() {\n                Sha256.prototype.finalize.call(this);\n                if (this.inner) {\n                    this.inner = false;\n                    var innerHash = this.array();\n                    Sha256.call(this, this.is224, this.sharedMemory);\n                    this.update(this.oKeyPad);\n                    this.update(innerHash);\n                    Sha256.prototype.finalize.call(this);\n                }\n            };\n            var exports = createMethod();\n            exports.sha256 = exports;\n            exports.sha224 = createMethod(true);\n            exports.sha256.hmac = createHmacMethod();\n            exports.sha224.hmac = createHmacMethod(true);\n            if (COMMON_JS) {\n                module.exports = exports;\n            } else {\n                root.sha256 = exports.sha256;\n                root.sha224 = exports.sha224;\n                if (AMD) {\n                    define(function() {\n                        return exports;\n                    });\n                }\n            }\n        })();\n    }\n});\n// node_modules/@dfinity/principal/lib/esm/utils/base32.js\nvar alphabet = \"abcdefghijklmnopqrstuvwxyz234567\";\nvar lookupTable = /* @__PURE__ */ Object.create(null);\nfor(let i = 0; i < alphabet.length; i++){\n    lookupTable[alphabet[i]] = i;\n}\nlookupTable[\"0\"] = lookupTable.o;\nlookupTable[\"1\"] = lookupTable.i;\nfunction encode(input) {\n    let skip = 0;\n    let bits = 0;\n    let output = \"\";\n    function encodeByte(byte) {\n        if (skip < 0) {\n            bits |= byte >> -skip;\n        } else {\n            bits = byte << skip & 248;\n        }\n        if (skip > 3) {\n            skip -= 8;\n            return 1;\n        }\n        if (skip < 4) {\n            output += alphabet[bits >> 3];\n            skip += 5;\n        }\n        return 0;\n    }\n    for(let i7 = 0; i7 < input.length;){\n        i7 += encodeByte(input[i7]);\n    }\n    return output + (skip < 0 ? alphabet[bits >> 3] : \"\");\n}\nfunction decode(input) {\n    let skip = 0;\n    let byte = 0;\n    const output = new Uint8Array(input.length * 4 / 3 | 0);\n    let o = 0;\n    function decodeChar(char) {\n        let val = lookupTable[char.toLowerCase()];\n        if (val === void 0) {\n            throw new Error(`Invalid character: ${JSON.stringify(char)}`);\n        }\n        val <<= 3;\n        byte |= val >>> skip;\n        skip += 5;\n        if (skip >= 8) {\n            output[o++] = byte;\n            skip -= 8;\n            if (skip > 0) {\n                byte = val << 5 - skip & 255;\n            } else {\n                byte = 0;\n            }\n        }\n    }\n    for (const c of input){\n        decodeChar(c);\n    }\n    return output.slice(0, o);\n}\n// node_modules/@dfinity/principal/lib/esm/utils/getCrc.js\nvar lookUpTable = new Uint32Array([\n    0,\n    1996959894,\n    3993919788,\n    2567524794,\n    124634137,\n    1886057615,\n    3915621685,\n    2657392035,\n    249268274,\n    2044508324,\n    3772115230,\n    2547177864,\n    162941995,\n    2125561021,\n    3887607047,\n    2428444049,\n    498536548,\n    1789927666,\n    4089016648,\n    2227061214,\n    450548861,\n    1843258603,\n    4107580753,\n    2211677639,\n    325883990,\n    1684777152,\n    4251122042,\n    2321926636,\n    335633487,\n    1661365465,\n    4195302755,\n    2366115317,\n    997073096,\n    1281953886,\n    3579855332,\n    2724688242,\n    1006888145,\n    1258607687,\n    3524101629,\n    2768942443,\n    901097722,\n    1119000684,\n    3686517206,\n    2898065728,\n    853044451,\n    1172266101,\n    3705015759,\n    2882616665,\n    651767980,\n    1373503546,\n    3369554304,\n    3218104598,\n    565507253,\n    1454621731,\n    3485111705,\n    3099436303,\n    671266974,\n    1594198024,\n    3322730930,\n    2970347812,\n    795835527,\n    1483230225,\n    3244367275,\n    3060149565,\n    1994146192,\n    31158534,\n    2563907772,\n    4023717930,\n    1907459465,\n    112637215,\n    2680153253,\n    3904427059,\n    2013776290,\n    251722036,\n    2517215374,\n    3775830040,\n    2137656763,\n    141376813,\n    2439277719,\n    3865271297,\n    1802195444,\n    476864866,\n    2238001368,\n    4066508878,\n    1812370925,\n    453092731,\n    2181625025,\n    4111451223,\n    1706088902,\n    314042704,\n    2344532202,\n    4240017532,\n    1658658271,\n    366619977,\n    2362670323,\n    4224994405,\n    1303535960,\n    984961486,\n    2747007092,\n    3569037538,\n    1256170817,\n    1037604311,\n    2765210733,\n    3554079995,\n    1131014506,\n    879679996,\n    2909243462,\n    3663771856,\n    1141124467,\n    855842277,\n    2852801631,\n    3708648649,\n    1342533948,\n    654459306,\n    3188396048,\n    3373015174,\n    1466479909,\n    544179635,\n    3110523913,\n    3462522015,\n    1591671054,\n    702138776,\n    2966460450,\n    3352799412,\n    1504918807,\n    783551873,\n    3082640443,\n    3233442989,\n    3988292384,\n    2596254646,\n    62317068,\n    1957810842,\n    3939845945,\n    2647816111,\n    81470997,\n    1943803523,\n    3814918930,\n    2489596804,\n    225274430,\n    2053790376,\n    3826175755,\n    2466906013,\n    167816743,\n    2097651377,\n    4027552580,\n    2265490386,\n    503444072,\n    1762050814,\n    4150417245,\n    2154129355,\n    426522225,\n    1852507879,\n    4275313526,\n    2312317920,\n    282753626,\n    1742555852,\n    4189708143,\n    2394877945,\n    397917763,\n    1622183637,\n    3604390888,\n    2714866558,\n    953729732,\n    1340076626,\n    3518719985,\n    2797360999,\n    1068828381,\n    1219638859,\n    3624741850,\n    2936675148,\n    906185462,\n    1090812512,\n    3747672003,\n    2825379669,\n    829329135,\n    1181335161,\n    3412177804,\n    3160834842,\n    628085408,\n    1382605366,\n    3423369109,\n    3138078467,\n    570562233,\n    1426400815,\n    3317316542,\n    2998733608,\n    733239954,\n    1555261956,\n    3268935591,\n    3050360625,\n    752459403,\n    1541320221,\n    2607071920,\n    3965973030,\n    1969922972,\n    40735498,\n    2617837225,\n    3943577151,\n    1913087877,\n    83908371,\n    2512341634,\n    3803740692,\n    2075208622,\n    213261112,\n    2463272603,\n    3855990285,\n    2094854071,\n    198958881,\n    2262029012,\n    4057260610,\n    1759359992,\n    534414190,\n    2176718541,\n    4139329115,\n    1873836001,\n    414664567,\n    2282248934,\n    4279200368,\n    1711684554,\n    285281116,\n    2405801727,\n    4167216745,\n    1634467795,\n    376229701,\n    2685067896,\n    3608007406,\n    1308918612,\n    956543938,\n    2808555105,\n    3495958263,\n    1231636301,\n    1047427035,\n    2932959818,\n    3654703836,\n    1088359270,\n    936918000,\n    2847714899,\n    3736837829,\n    1202900863,\n    817233897,\n    3183342108,\n    3401237130,\n    1404277552,\n    615818150,\n    3134207493,\n    3453421203,\n    1423857449,\n    601450431,\n    3009837614,\n    3294710456,\n    1567103746,\n    711928724,\n    3020668471,\n    3272380065,\n    1510334235,\n    755167117\n]);\nfunction getCrc32(buf) {\n    const b = new Uint8Array(buf);\n    let crc = -1;\n    for(let i8 = 0; i8 < b.length; i8++){\n        const byte = b[i8];\n        const t = (byte ^ crc) & 255;\n        crc = lookUpTable[t] ^ crc >>> 8;\n    }\n    return (crc ^ -1) >>> 0;\n}\n// node_modules/@dfinity/principal/lib/esm/utils/sha224.js\nvar import_js_sha256 = __toESM(require_sha256());\nfunction sha224(data) {\n    const shaObj = import_js_sha256.sha224.create();\n    shaObj.update(data);\n    return new Uint8Array(shaObj.array());\n}\n// node_modules/@dfinity/principal/lib/esm/index.js\nvar SELF_AUTHENTICATING_SUFFIX = 2;\nvar ANONYMOUS_SUFFIX = 4;\nvar MANAGEMENT_CANISTER_PRINCIPAL_HEX_STR = \"aaaaa-aa\";\nvar fromHexString = (hexString)=>{\n    var _a;\n    return new Uint8Array(((_a = hexString.match(/.{1,2}/g)) !== null && _a !== void 0 ? _a : []).map((byte)=>parseInt(byte, 16)\n    ));\n};\nvar toHexString = (bytes)=>bytes.reduce((str, byte)=>str + byte.toString(16).padStart(2, \"0\")\n    , \"\")\n;\nvar Principal = class {\n    static anonymous() {\n        return new this(new Uint8Array([\n            ANONYMOUS_SUFFIX\n        ]));\n    }\n    static managementCanister() {\n        return this.fromHex(MANAGEMENT_CANISTER_PRINCIPAL_HEX_STR);\n    }\n    static selfAuthenticating(publicKey) {\n        const sha = sha224(publicKey);\n        return new this(new Uint8Array([\n            ...sha,\n            SELF_AUTHENTICATING_SUFFIX\n        ]));\n    }\n    static from(other) {\n        if (typeof other === \"string\") {\n            return Principal.fromText(other);\n        } else if (typeof other === \"object\" && other !== null && other._isPrincipal === true) {\n            return new Principal(other._arr);\n        }\n        throw new Error(`Impossible to convert ${JSON.stringify(other)} to Principal.`);\n    }\n    static fromHex(hex) {\n        return new this(fromHexString(hex));\n    }\n    static fromText(text) {\n        const canisterIdNoDash = text.toLowerCase().replace(/-/g, \"\");\n        let arr = decode(canisterIdNoDash);\n        arr = arr.slice(4, arr.length);\n        const principal = new this(arr);\n        if (principal.toText() !== text) {\n            throw new Error(`Principal \"${principal.toText()}\" does not have a valid checksum (original value \"${text}\" may not be a valid Principal ID).`);\n        }\n        return principal;\n    }\n    static fromUint8Array(arr) {\n        return new this(arr);\n    }\n    isAnonymous() {\n        return this._arr.byteLength === 1 && this._arr[0] === ANONYMOUS_SUFFIX;\n    }\n    toUint8Array() {\n        return this._arr;\n    }\n    toHex() {\n        return toHexString(this._arr).toUpperCase();\n    }\n    toText() {\n        const checksumArrayBuf = new ArrayBuffer(4);\n        const view = new DataView(checksumArrayBuf);\n        view.setUint32(0, getCrc32(this._arr));\n        const checksum = new Uint8Array(checksumArrayBuf);\n        const bytes = Uint8Array.from(this._arr);\n        const array = new Uint8Array([\n            ...checksum,\n            ...bytes\n        ]);\n        const result = encode(array);\n        const matches = result.match(/.{1,5}/g);\n        if (!matches) {\n            throw new Error();\n        }\n        return matches.join(\"-\");\n    }\n    toString() {\n        return this.toText();\n    }\n    compareTo(other) {\n        for(let i9 = 0; i9 < Math.min(this._arr.length, other._arr.length); i9++){\n            if (this._arr[i9] < other._arr[i9]) return \"lt\";\n            else if (this._arr[i9] > other._arr[i9]) return \"gt\";\n        }\n        if (this._arr.length < other._arr.length) return \"lt\";\n        if (this._arr.length > other._arr.length) return \"gt\";\n        return \"eq\";\n    }\n    ltEq(other) {\n        const cmp = this.compareTo(other);\n        return cmp == \"lt\" || cmp == \"eq\";\n    }\n    gtEq(other) {\n        const cmp = this.compareTo(other);\n        return cmp == \"gt\" || cmp == \"eq\";\n    }\n    constructor(_arr){\n        this._arr = _arr;\n        this._isPrincipal = true;\n    }\n};\nexports.Principal = Principal;\n// node_modules/azle/node_modules/@dfinity/principal/lib/esm/utils/base32.js\nvar alphabet2 = \"abcdefghijklmnopqrstuvwxyz234567\";\nvar lookupTable2 = /* @__PURE__ */ Object.create(null);\nfor(let i1 = 0; i1 < alphabet2.length; i1++){\n    lookupTable2[alphabet2[i1]] = i1;\n}\nlookupTable2[\"0\"] = lookupTable2.o;\nlookupTable2[\"1\"] = lookupTable2.i;\n// node_modules/azle/node_modules/@dfinity/principal/lib/esm/utils/getCrc.js\nvar lookUpTable2 = new Uint32Array([\n    0,\n    1996959894,\n    3993919788,\n    2567524794,\n    124634137,\n    1886057615,\n    3915621685,\n    2657392035,\n    249268274,\n    2044508324,\n    3772115230,\n    2547177864,\n    162941995,\n    2125561021,\n    3887607047,\n    2428444049,\n    498536548,\n    1789927666,\n    4089016648,\n    2227061214,\n    450548861,\n    1843258603,\n    4107580753,\n    2211677639,\n    325883990,\n    1684777152,\n    4251122042,\n    2321926636,\n    335633487,\n    1661365465,\n    4195302755,\n    2366115317,\n    997073096,\n    1281953886,\n    3579855332,\n    2724688242,\n    1006888145,\n    1258607687,\n    3524101629,\n    2768942443,\n    901097722,\n    1119000684,\n    3686517206,\n    2898065728,\n    853044451,\n    1172266101,\n    3705015759,\n    2882616665,\n    651767980,\n    1373503546,\n    3369554304,\n    3218104598,\n    565507253,\n    1454621731,\n    3485111705,\n    3099436303,\n    671266974,\n    1594198024,\n    3322730930,\n    2970347812,\n    795835527,\n    1483230225,\n    3244367275,\n    3060149565,\n    1994146192,\n    31158534,\n    2563907772,\n    4023717930,\n    1907459465,\n    112637215,\n    2680153253,\n    3904427059,\n    2013776290,\n    251722036,\n    2517215374,\n    3775830040,\n    2137656763,\n    141376813,\n    2439277719,\n    3865271297,\n    1802195444,\n    476864866,\n    2238001368,\n    4066508878,\n    1812370925,\n    453092731,\n    2181625025,\n    4111451223,\n    1706088902,\n    314042704,\n    2344532202,\n    4240017532,\n    1658658271,\n    366619977,\n    2362670323,\n    4224994405,\n    1303535960,\n    984961486,\n    2747007092,\n    3569037538,\n    1256170817,\n    1037604311,\n    2765210733,\n    3554079995,\n    1131014506,\n    879679996,\n    2909243462,\n    3663771856,\n    1141124467,\n    855842277,\n    2852801631,\n    3708648649,\n    1342533948,\n    654459306,\n    3188396048,\n    3373015174,\n    1466479909,\n    544179635,\n    3110523913,\n    3462522015,\n    1591671054,\n    702138776,\n    2966460450,\n    3352799412,\n    1504918807,\n    783551873,\n    3082640443,\n    3233442989,\n    3988292384,\n    2596254646,\n    62317068,\n    1957810842,\n    3939845945,\n    2647816111,\n    81470997,\n    1943803523,\n    3814918930,\n    2489596804,\n    225274430,\n    2053790376,\n    3826175755,\n    2466906013,\n    167816743,\n    2097651377,\n    4027552580,\n    2265490386,\n    503444072,\n    1762050814,\n    4150417245,\n    2154129355,\n    426522225,\n    1852507879,\n    4275313526,\n    2312317920,\n    282753626,\n    1742555852,\n    4189708143,\n    2394877945,\n    397917763,\n    1622183637,\n    3604390888,\n    2714866558,\n    953729732,\n    1340076626,\n    3518719985,\n    2797360999,\n    1068828381,\n    1219638859,\n    3624741850,\n    2936675148,\n    906185462,\n    1090812512,\n    3747672003,\n    2825379669,\n    829329135,\n    1181335161,\n    3412177804,\n    3160834842,\n    628085408,\n    1382605366,\n    3423369109,\n    3138078467,\n    570562233,\n    1426400815,\n    3317316542,\n    2998733608,\n    733239954,\n    1555261956,\n    3268935591,\n    3050360625,\n    752459403,\n    1541320221,\n    2607071920,\n    3965973030,\n    1969922972,\n    40735498,\n    2617837225,\n    3943577151,\n    1913087877,\n    83908371,\n    2512341634,\n    3803740692,\n    2075208622,\n    213261112,\n    2463272603,\n    3855990285,\n    2094854071,\n    198958881,\n    2262029012,\n    4057260610,\n    1759359992,\n    534414190,\n    2176718541,\n    4139329115,\n    1873836001,\n    414664567,\n    2282248934,\n    4279200368,\n    1711684554,\n    285281116,\n    2405801727,\n    4167216745,\n    1634467795,\n    376229701,\n    2685067896,\n    3608007406,\n    1308918612,\n    956543938,\n    2808555105,\n    3495958263,\n    1231636301,\n    1047427035,\n    2932959818,\n    3654703836,\n    1088359270,\n    936918000,\n    2847714899,\n    3736837829,\n    1202900863,\n    817233897,\n    3183342108,\n    3401237130,\n    1404277552,\n    615818150,\n    3134207493,\n    3453421203,\n    1423857449,\n    601450431,\n    3009837614,\n    3294710456,\n    1567103746,\n    711928724,\n    3020668471,\n    3272380065,\n    1510334235,\n    755167117\n]);\n// node_modules/azle/node_modules/@dfinity/principal/lib/esm/utils/sha224.js\nvar import_js_sha2562 = __toESM(require_sha256());\nvar _ic;\n// node_modules/azle/index.ts\nvar ic = (_ic = globalThis.ic) !== null && _ic !== void 0 ? _ic : {};\n// src/index.ts\nfunction hello_world() {\n    return \"Hello world!\";\n}\nexports.hello_world = hello_world;\n\n        " ;
fn _azle_custom_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    RNG_REF_CELL.with(|rng_ref_cell| {
        let mut rng = rng_ref_cell.borrow_mut();
        rng.fill(_buf);
    });
    Ok(())
}
getrandom::register_custom_getrandom!(_azle_custom_getrandom);
pub trait CdkActTryIntoVmValue<Context, VmValue> {
    fn try_into_vm_value(self, context: Context) -> Result<VmValue, CdkActTryIntoVmValueError>;
}
#[derive(Debug)]
pub struct CdkActTryIntoVmValueError(pub String);
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for () {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(boa_engine::JsValue::Null)
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for bool {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for String {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue>
    for ic_cdk::export::candid::Empty
{
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Err(CdkActTryIntoVmValueError(
            "Empty cannot be converted into JsValue".to_string(),
        ))
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue>
    for ic_cdk::export::candid::Reserved
{
    fn try_into_vm_value(
        self,
        _: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(boa_engine::JsValue::Null)
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue>
    for ic_cdk::export::candid::Func
{
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(boa_engine::object::builtins::JsArray::from_iter(
            [
                self.principal.try_into_vm_value(context).unwrap(),
                self.method.into(),
            ],
            context,
        )
        .into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue>
    for ic_cdk::export::Principal
{
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        let exports_js_value = _azle_unwrap_boa_result(context.eval("exports"), context);
        let exports_js_object = exports_js_value.as_object().unwrap();
        let principal_class_js_value = exports_js_object.get("Principal", context).unwrap();
        let principal_class_js_object = principal_class_js_value.as_object().unwrap();
        let from_text_js_value = principal_class_js_object.get("fromText", context).unwrap();
        let from_text_js_object = from_text_js_value.as_object().unwrap();
        let principal_js_value = _azle_unwrap_boa_result(
            from_text_js_object.call(&principal_class_js_value, &[self.to_text().into()], context),
            context,
        );
        Ok(principal_js_value)
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue>
    for ic_cdk::timer::TimerId
{
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        let timer_id_as_u64 = self.data().as_ffi();
        Ok(boa_engine::JsValue::BigInt(timer_id_as_u64.into()))
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue>
    for ic_cdk::api::stable::StableMemoryError
{
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        match self {
            ic_cdk::api::stable::StableMemoryError::OutOfMemory => {
                Ok(boa_engine::object::ObjectInitializer::new(context)
                    .property(
                        "OutOfMemory",
                        boa_engine::JsValue::Null,
                        boa_engine::property::Attribute::all(),
                    )
                    .build()
                    .into())
            }
            ic_cdk::api::stable::StableMemoryError::OutOfBounds => {
                Ok(boa_engine::object::ObjectInitializer::new(context)
                    .property(
                        "OutOfBounds",
                        boa_engine::JsValue::Null,
                        boa_engine::property::Attribute::all(),
                    )
                    .build()
                    .into())
            }
        }
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue>
    for ic_stable_structures::btreemap::InsertError
{
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        match self {
            ic_stable_structures::btreemap::InsertError::KeyTooLarge { given, max } => {
                let given_js_value = given.try_into_vm_value(context).unwrap();
                let max_js_value = max.try_into_vm_value(context).unwrap();
                let key_too_large_object = boa_engine::object::ObjectInitializer::new(context)
                    .property(
                        "given",
                        given_js_value,
                        boa_engine::property::Attribute::all(),
                    )
                    .property("max", max_js_value, boa_engine::property::Attribute::all())
                    .build();
                Ok(boa_engine::object::ObjectInitializer::new(context)
                    .property(
                        "KeyTooLarge",
                        key_too_large_object,
                        boa_engine::property::Attribute::all(),
                    )
                    .build()
                    .into())
            }
            ic_stable_structures::btreemap::InsertError::ValueTooLarge { given, max } => {
                let given_js_value = given.try_into_vm_value(context).unwrap();
                let max_js_value = max.try_into_vm_value(context).unwrap();
                let value_too_large_object = boa_engine::object::ObjectInitializer::new(context)
                    .property(
                        "given",
                        given_js_value,
                        boa_engine::property::Attribute::all(),
                    )
                    .property("max", max_js_value, boa_engine::property::Attribute::all())
                    .build();
                Ok(boa_engine::object::ObjectInitializer::new(context)
                    .property(
                        "ValueTooLarge",
                        value_too_large_object,
                        boa_engine::property::Attribute::all(),
                    )
                    .build()
                    .into())
            }
        }
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue>
    for ic_cdk::api::call::RejectionCode
{
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        match self {
            ic_cdk::api::call::RejectionCode::NoError => {
                Ok(boa_engine::object::ObjectInitializer::new(context)
                    .property(
                        "NoError",
                        boa_engine::JsValue::Null,
                        boa_engine::property::Attribute::all(),
                    )
                    .build()
                    .into())
            }
            ic_cdk::api::call::RejectionCode::SysFatal => {
                Ok(boa_engine::object::ObjectInitializer::new(context)
                    .property(
                        "SysFatal",
                        boa_engine::JsValue::Null,
                        boa_engine::property::Attribute::all(),
                    )
                    .build()
                    .into())
            }
            ic_cdk::api::call::RejectionCode::SysTransient => {
                Ok(boa_engine::object::ObjectInitializer::new(context)
                    .property(
                        "SysTransient",
                        boa_engine::JsValue::Null,
                        boa_engine::property::Attribute::all(),
                    )
                    .build()
                    .into())
            }
            ic_cdk::api::call::RejectionCode::DestinationInvalid => {
                Ok(boa_engine::object::ObjectInitializer::new(context)
                    .property(
                        "DestinationInvalid",
                        boa_engine::JsValue::Null,
                        boa_engine::property::Attribute::all(),
                    )
                    .build()
                    .into())
            }
            ic_cdk::api::call::RejectionCode::CanisterReject => {
                Ok(boa_engine::object::ObjectInitializer::new(context)
                    .property(
                        "CanisterReject",
                        boa_engine::JsValue::Null,
                        boa_engine::property::Attribute::all(),
                    )
                    .build()
                    .into())
            }
            ic_cdk::api::call::RejectionCode::CanisterError => {
                Ok(boa_engine::object::ObjectInitializer::new(context)
                    .property(
                        "CanisterError",
                        boa_engine::JsValue::Null,
                        boa_engine::property::Attribute::all(),
                    )
                    .build()
                    .into())
            }
            ic_cdk::api::call::RejectionCode::Unknown => {
                Ok(boa_engine::object::ObjectInitializer::new(context)
                    .property(
                        "Unknown",
                        boa_engine::JsValue::Null,
                        boa_engine::property::Attribute::all(),
                    )
                    .build()
                    .into())
            }
        }
    }
}
impl<T> CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for (T,)
where
    T: for<'a> CdkActTryIntoVmValue<&'a mut boa_engine::Context, boa_engine::JsValue>,
{
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.0.try_into_vm_value(context).unwrap())
    }
}
impl<T> CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for Box<T>
where
    T: for<'a> CdkActTryIntoVmValue<&'a mut boa_engine::Context, boa_engine::JsValue>,
{
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok((*self).try_into_vm_value(context).unwrap())
    }
}
impl<T> CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for Option<T>
where
    T: for<'a> CdkActTryIntoVmValue<&'a mut boa_engine::Context, boa_engine::JsValue>,
{
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        match self {
            Some(value) => Ok(value.try_into_vm_value(context).unwrap()),
            None => Ok(boa_engine::JsValue::Null),
        }
    }
}
impl<T, K> CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for Result<T, K>
where
    T: for<'a> CdkActTryIntoVmValue<&'a mut boa_engine::Context, boa_engine::JsValue>,
    K: for<'a> CdkActTryIntoVmValue<&'a mut boa_engine::Context, boa_engine::JsValue>,
{
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        match self {
            Ok(ok) => {
                let ok_js_value = ok.try_into_vm_value(context).unwrap();
                let result_js_object = boa_engine::object::ObjectInitializer::new(context)
                    .property("ok", ok_js_value, boa_engine::property::Attribute::all())
                    .build();
                let result_js_value = result_js_object.into();
                Ok(result_js_value)
            }
            Err(err) => {
                let err_js_value = err.try_into_vm_value(context).unwrap();
                let result_js_object = boa_engine::object::ObjectInitializer::new(context)
                    .property("err", err_js_value, boa_engine::property::Attribute::all())
                    .build();
                let result_js_value = result_js_object.into();
                Ok(result_js_value)
            }
        }
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for f64 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for f32 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue>
    for ic_cdk::export::candid::Int
{
    fn try_into_vm_value(
        self,
        _: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(boa_engine::JsValue::BigInt(
            boa_engine::bigint::JsBigInt::new(self.0),
        ))
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for i128 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(boa_engine::bigint::JsBigInt::new(self).into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for i64 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(boa_engine::JsValue::BigInt(self.into()))
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for i32 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for i16 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for i8 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue>
    for ic_cdk::export::candid::Nat
{
    fn try_into_vm_value(
        self,
        _: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(boa_engine::JsValue::BigInt(
            boa_engine::bigint::JsBigInt::from_string(&self.0.to_string()).unwrap(),
        ))
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for u128 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(boa_engine::bigint::JsBigInt::new(self).into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for u64 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(boa_engine::JsValue::BigInt(self.into()))
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for usize {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for u32 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for u16 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for u8 {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
trait AzleTryIntoVec {}
impl AzleTryIntoVec for () {}
impl AzleTryIntoVec for bool {}
impl AzleTryIntoVec for String {}
impl AzleTryIntoVec for ic_cdk::export::candid::Empty {}
impl AzleTryIntoVec for ic_cdk::export::candid::Reserved {}
impl AzleTryIntoVec for ic_cdk::export::candid::Func {}
impl AzleTryIntoVec for ic_cdk::export::Principal {}
impl AzleTryIntoVec for ic_cdk::timer::TimerId {}
impl AzleTryIntoVec for ic_cdk::api::call::RejectionCode {}
impl AzleTryIntoVec for f64 {}
impl AzleTryIntoVec for f32 {}
impl AzleTryIntoVec for ic_cdk::export::candid::Int {}
impl AzleTryIntoVec for i128 {}
impl AzleTryIntoVec for i64 {}
impl AzleTryIntoVec for i32 {}
impl AzleTryIntoVec for i16 {}
impl AzleTryIntoVec for i8 {}
impl AzleTryIntoVec for ic_cdk::export::candid::Nat {}
impl AzleTryIntoVec for u128 {}
impl AzleTryIntoVec for u64 {}
impl AzleTryIntoVec for usize {}
impl AzleTryIntoVec for u32 {}
impl AzleTryIntoVec for u16 {}
impl<T> AzleTryIntoVec for Option<T> {}
impl<T> AzleTryIntoVec for Vec<T> {}
impl<T> CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for Vec<T>
where
    T: AzleTryIntoVec,
    T: for<'a> CdkActTryIntoVmValue<&'a mut boa_engine::Context, boa_engine::JsValue>,
{
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        try_into_vm_value_generic_array(self, context)
    }
}
impl CdkActTryIntoVmValue<&mut boa_engine::Context, boa_engine::JsValue> for Vec<u8> {
    fn try_into_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError> {
        Ok(self.into())
    }
}
fn try_into_vm_value_generic_array<T>(
    generic_array: Vec<T>,
    context: &mut boa_engine::Context,
) -> Result<boa_engine::JsValue, CdkActTryIntoVmValueError>
where
    T: for<'a> CdkActTryIntoVmValue<&'a mut boa_engine::Context, boa_engine::JsValue>,
{
    let js_values = generic_array
        .into_iter()
        .map(|item| item.try_into_vm_value(context).unwrap())
        .collect::<Vec<boa_engine::JsValue>>();
    Ok(boa_engine::object::builtins::JsArray::from_iter(js_values, context).into())
}
pub trait CdkActTryFromVmValue<T, Context> {
    fn try_from_vm_value(self, context: Context) -> Result<T, CdkActTryFromVmValueError>;
}
#[derive(Debug)]
pub struct CdkActTryFromVmValueError(pub String);
impl CdkActTryFromVmValue<(), &mut boa_engine::Context> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<(), CdkActTryFromVmValueError> {
        if self.is_null() == true || self.is_undefined() == true {
            Ok(())
        } else {
            Err(CdkActTryFromVmValueError(
                "JsValue is not null or undefined".to_string(),
            ))
        }
    }
}
impl CdkActTryFromVmValue<bool, &mut boa_engine::Context> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<bool, CdkActTryFromVmValueError> {
        match self.as_boolean() {
            Some(value) => Ok(value),
            None => Err(CdkActTryFromVmValueError(
                "JsValue is not a boolean".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<String, &mut boa_engine::Context> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<String, CdkActTryFromVmValueError> {
        match self.as_string() {
            Some(value) => Ok(value.to_std_string().unwrap()),
            None => Err(CdkActTryFromVmValueError(
                "JsValue is not a string".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<ic_cdk::export::candid::Empty, &mut boa_engine::Context>
    for boa_engine::JsValue
{
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<ic_cdk::export::candid::Empty, CdkActTryFromVmValueError> {
        Err(CdkActTryFromVmValueError(
            "JsValue cannot be converted into type 'empty'".to_string(),
        ))
    }
}
impl CdkActTryFromVmValue<ic_cdk::export::candid::Reserved, &mut boa_engine::Context>
    for boa_engine::JsValue
{
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<ic_cdk::export::candid::Reserved, CdkActTryFromVmValueError> {
        Ok(ic_cdk::export::candid::Reserved)
    }
}
impl CdkActTryFromVmValue<ic_cdk::export::candid::Func, &mut boa_engine::Context>
    for boa_engine::JsValue
{
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<ic_cdk::export::candid::Func, CdkActTryFromVmValueError> {
        match self.as_object() {
            Some(js_object) => match (js_object.get("0", context), js_object.get("1", context)) {
                (Ok(principal_js_value), Ok(canister_method_text)) => {
                    match (
                        principal_js_value.try_from_vm_value(&mut *context),
                        canister_method_text.try_from_vm_value(&mut *context),
                    ) {
                        (Ok(principal), Ok(canister_method_string)) => {
                            Ok(ic_cdk::export::candid::Func {
                                principal,
                                method: canister_method_string,
                            })
                        }
                        _ => Err(CdkActTryFromVmValueError(
                            "principal could not be created or canister method not a string"
                                .to_string(),
                        )),
                    }
                }
                _ => Err(CdkActTryFromVmValueError(
                    "Could not retrieve index 0 or 1".to_string(),
                )),
            },
            None => Err(CdkActTryFromVmValueError(
                "JsValue is not an object".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<ic_cdk::export::Principal, &mut boa_engine::Context>
    for boa_engine::JsValue
{
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<ic_cdk::export::Principal, CdkActTryFromVmValueError> {
        match self.as_object() {
            Some(principal_js_object) => match principal_js_object.get("toText", context) {
                Ok(principal_to_text_function_js_value) => {
                    match principal_to_text_function_js_value.as_object() {
                        Some(principal_to_text_function_js_object) => {
                            match principal_to_text_function_js_object.call(&self, &[], context) {
                                Ok(principal_string_js_value) => {
                                    match principal_string_js_value.as_string() {
                                        Some(principal_js_string) => {
                                            match ic_cdk::export::Principal::from_text(
                                                principal_js_string.to_std_string().unwrap(),
                                            ) {
                                                Ok(principal) => Ok(principal),
                                                Err(err) => {
                                                    Err(CdkActTryFromVmValueError(err.to_string()))
                                                }
                                            }
                                        }
                                        None => Err(CdkActTryFromVmValueError(
                                            "JsValue is not a string".to_string(),
                                        )),
                                    }
                                }
                                Err(err_js_error) => {
                                    let err_js_value = err_js_error.to_opaque(context);
                                    let err_js_object = err_js_value.as_object().unwrap();
                                    let err_name: String = err_js_object
                                        .get("name", &mut *context)
                                        .unwrap()
                                        .try_from_vm_value(&mut *context)
                                        .unwrap();
                                    let err_message: String = err_js_object
                                        .get("message", &mut *context)
                                        .unwrap()
                                        .try_from_vm_value(&mut *context)
                                        .unwrap();
                                    Err(CdkActTryFromVmValueError(format!(
                                        "{name}: {message}",
                                        name = err_name,
                                        message = err_message
                                    )))
                                }
                            }
                        }
                        None => Err(CdkActTryFromVmValueError(
                            "JsValue is not an object".to_string(),
                        )),
                    }
                }
                Err(err) => Err(CdkActTryFromVmValueError(
                    "principal_js_object.get(\"toText\", context) failed".to_string(),
                )),
            },
            None => Err(CdkActTryFromVmValueError(
                "JsValue is not an object".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<ic_cdk::timer::TimerId, &mut boa_engine::Context>
    for boa_engine::JsValue
{
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<ic_cdk::timer::TimerId, CdkActTryFromVmValueError> {
        let js_value_as_u64: u64 = self.try_from_vm_value(context).unwrap();
        Ok(ic_cdk::timer::TimerId::from(slotmap::KeyData::from_ffi(
            js_value_as_u64,
        )))
    }
}
impl CdkActTryFromVmValue<f64, &mut boa_engine::Context> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<f64, CdkActTryFromVmValueError> {
        match self.as_number() {
            Some(value) => Ok(value),
            None => Err(CdkActTryFromVmValueError(
                "JsValue is not a number".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<f32, &mut boa_engine::Context> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<f32, CdkActTryFromVmValueError> {
        match self.as_number() {
            Some(value) => Ok(value as f32),
            None => Err(CdkActTryFromVmValueError(
                "JsValue is not a number".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<ic_cdk::export::candid::Int, &mut boa_engine::Context>
    for boa_engine::JsValue
{
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<ic_cdk::export::candid::Int, CdkActTryFromVmValueError> {
        match self.as_bigint() {
            Some(value) => Ok(ic_cdk::export::candid::Int::from_str(&value.to_string()).unwrap()),
            None => Err(CdkActTryFromVmValueError(
                "JsValue is not a bigint".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<i128, &mut boa_engine::Context> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<i128, CdkActTryFromVmValueError> {
        match self.as_bigint() {
            Some(value) => {
                let value_i128_result = value.to_string().parse::<i128>();
                match value_i128_result {
                    Ok(value_i128) => Ok(value_i128),
                    Err(_) => Err(CdkActTryFromVmValueError(
                        "Could not parse bigint to i128".to_string(),
                    )),
                }
            }
            None => Err(CdkActTryFromVmValueError(
                "JsValue is not a bigint".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<i64, &mut boa_engine::Context> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<i64, CdkActTryFromVmValueError> {
        match self.as_bigint() {
            Some(value) => {
                let value_i64_result = value.to_string().parse::<i64>();
                match value_i64_result {
                    Ok(value_i64) => Ok(value_i64),
                    Err(_) => Err(CdkActTryFromVmValueError(
                        "Could not parse bigint to i64".to_string(),
                    )),
                }
            }
            None => Err(CdkActTryFromVmValueError(
                "JsValue is not a bigint".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<i32, &mut boa_engine::Context> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<i32, CdkActTryFromVmValueError> {
        match self.as_number() {
            Some(value) => Ok(value as i32),
            None => Err(CdkActTryFromVmValueError(
                "JsValue is not a number".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<i16, &mut boa_engine::Context> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<i16, CdkActTryFromVmValueError> {
        match self.as_number() {
            Some(value) => Ok(value as i16),
            None => Err(CdkActTryFromVmValueError(
                "JsValue is not a number".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<i8, &mut boa_engine::Context> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<i8, CdkActTryFromVmValueError> {
        match self.as_number() {
            Some(value) => Ok(value as i8),
            None => Err(CdkActTryFromVmValueError(
                "JsValue is not a number".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<ic_cdk::export::candid::Nat, &mut boa_engine::Context>
    for boa_engine::JsValue
{
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<ic_cdk::export::candid::Nat, CdkActTryFromVmValueError> {
        match self.as_bigint() {
            Some(value) => Ok(ic_cdk::export::candid::Nat::from_str(&value.to_string()).unwrap()),
            None => Err(CdkActTryFromVmValueError(
                "JsValue is not a bigint".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<u128, &mut boa_engine::Context> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<u128, CdkActTryFromVmValueError> {
        match self.as_bigint() {
            Some(value) => {
                let value_u128_result = value.to_string().parse::<u128>();
                match value_u128_result {
                    Ok(value_u128) => Ok(value_u128),
                    Err(_) => Err(CdkActTryFromVmValueError(
                        "Could not parse bigint to u128".to_string(),
                    )),
                }
            }
            None => Err(CdkActTryFromVmValueError(
                "JsValue is not a bigint".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<u64, &mut boa_engine::Context> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<u64, CdkActTryFromVmValueError> {
        match self.as_bigint() {
            Some(value) => {
                let value_u64_result = value.to_string().parse::<u64>();
                match value_u64_result {
                    Ok(value_u64) => Ok(value_u64),
                    Err(_) => Err(CdkActTryFromVmValueError(
                        "Could not parse bigint to u64".to_string(),
                    )),
                }
            }
            None => Err(CdkActTryFromVmValueError(
                "JsValue is not a bigint".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<u32, &mut boa_engine::Context> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<u32, CdkActTryFromVmValueError> {
        match self.as_number() {
            Some(value) => Ok(value as u32),
            None => Err(CdkActTryFromVmValueError(
                "JsValue is not a number".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<u16, &mut boa_engine::Context> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<u16, CdkActTryFromVmValueError> {
        match self.as_number() {
            Some(value) => Ok(value as u16),
            None => Err(CdkActTryFromVmValueError(
                "JsValue is not a number".to_string(),
            )),
        }
    }
}
impl CdkActTryFromVmValue<u8, &mut boa_engine::Context> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<u8, CdkActTryFromVmValueError> {
        match self.as_number() {
            Some(value) => Ok(value as u8),
            None => Err(CdkActTryFromVmValueError(
                "JsValue is not a number".to_string(),
            )),
        }
    }
}
impl<T> CdkActTryFromVmValue<(T,), &mut boa_engine::Context> for boa_engine::JsValue
where
    boa_engine::JsValue: for<'a> CdkActTryFromVmValue<T, &'a mut boa_engine::Context>,
{
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<(T,), CdkActTryFromVmValueError> {
        Ok((self.try_from_vm_value(context).unwrap(),))
    }
}
impl<T> CdkActTryFromVmValue<Box<T>, &mut boa_engine::Context> for boa_engine::JsValue
where
    boa_engine::JsValue: for<'a> CdkActTryFromVmValue<T, &'a mut boa_engine::Context>,
{
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<Box<T>, CdkActTryFromVmValueError> {
        match self.try_from_vm_value(context) {
            Ok(value) => Ok(Box::new(value)),
            Err(err) => Err(err),
        }
    }
}
impl<T> CdkActTryFromVmValue<Option<T>, &mut boa_engine::Context> for boa_engine::JsValue
where
    boa_engine::JsValue: for<'a> CdkActTryFromVmValue<T, &'a mut boa_engine::Context>,
{
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<Option<T>, CdkActTryFromVmValueError> {
        if self.is_null() {
            Ok(None)
        } else {
            match self.try_from_vm_value(context) {
                Ok(value) => Ok(Some(value)),
                Err(err) => Err(err),
            }
        }
    }
}
trait AzleTryFromVec {}
impl AzleTryFromVec for () {}
impl AzleTryFromVec for bool {}
impl AzleTryFromVec for String {}
impl AzleTryFromVec for ic_cdk::export::candid::Empty {}
impl AzleTryFromVec for ic_cdk::export::candid::Reserved {}
impl AzleTryFromVec for ic_cdk::export::candid::Func {}
impl AzleTryFromVec for ic_cdk::export::Principal {}
impl AzleTryFromVec for ic_cdk::timer::TimerId {}
impl AzleTryFromVec for f64 {}
impl AzleTryFromVec for f32 {}
impl AzleTryFromVec for ic_cdk::export::candid::Int {}
impl AzleTryFromVec for i128 {}
impl AzleTryFromVec for i64 {}
impl AzleTryFromVec for i32 {}
impl AzleTryFromVec for i16 {}
impl AzleTryFromVec for i8 {}
impl AzleTryFromVec for ic_cdk::export::candid::Nat {}
impl AzleTryFromVec for u128 {}
impl AzleTryFromVec for u64 {}
impl AzleTryFromVec for u32 {}
impl AzleTryFromVec for u16 {}
impl<T> AzleTryFromVec for Option<T> {}
impl<T> AzleTryFromVec for Vec<T> {}
impl<T> CdkActTryFromVmValue<Vec<T>, &mut boa_engine::Context> for boa_engine::JsValue
where
    T: AzleTryFromVec,
    boa_engine::JsValue: for<'a> CdkActTryFromVmValue<T, &'a mut boa_engine::Context>,
{
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<Vec<T>, CdkActTryFromVmValueError> {
        try_from_vm_value_generic_array::<T>(self, context)
    }
}
impl CdkActTryFromVmValue<Vec<u8>, &mut boa_engine::Context> for boa_engine::JsValue {
    fn try_from_vm_value(
        self,
        context: &mut boa_engine::Context,
    ) -> Result<Vec<u8>, CdkActTryFromVmValueError> {
        Ok(self
            .as_object()
            .unwrap()
            .borrow()
            .as_typed_array()
            .unwrap()
            .viewed_array_buffer()
            .unwrap()
            .borrow()
            .as_array_buffer()
            .unwrap()
            .array_buffer_data
            .clone()
            .unwrap())
    }
}
fn try_from_vm_value_generic_array<T>(
    js_value: boa_engine::JsValue,
    context: &mut boa_engine::Context,
) -> Result<Vec<T>, CdkActTryFromVmValueError>
where
    boa_engine::JsValue: for<'a> CdkActTryFromVmValue<T, &'a mut boa_engine::Context>,
{
    match js_value.as_object() {
        Some(js_object) => {
            if js_object.is_array() {
                let mut processing: bool = true;
                let mut index: usize = 0;
                let mut result = vec![];
                while processing == true {
                    match js_object.get(index, context) {
                        Ok(js_value) => {
                            if js_value.is_undefined() {
                                processing = false;
                            } else {
                                match js_value.try_from_vm_value(context) {
                                    Ok(value) => {
                                        result.push(value);
                                        index += 1;
                                    }
                                    Err(err) => {
                                        return Err(err);
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            return Err(CdkActTryFromVmValueError(
                                "Item at array index does not exist".to_string(),
                            ))
                        }
                    }
                }
                Ok(result)
            } else {
                Err(CdkActTryFromVmValueError(
                    "JsObject is not an array".to_string(),
                ))
            }
        }
        None => Err(CdkActTryFromVmValueError(
            "JsValue is not an object".to_string(),
        )),
    }
}
#[ic_cdk_macros::init]
#[candid::candid_method(init)]
fn _azle_init() {
    BOA_CONTEXT_REF_CELL.with(|box_context_ref_cell| {
        let mut _azle_boa_context = box_context_ref_cell.borrow_mut();
        METHOD_NAME_REF_CELL.with(|method_name_ref_cell| {
            let mut method_name_mut = method_name_ref_cell.borrow_mut();
            *method_name_mut = "DOES_NOT_EXIST".to_string()
        });
        _azle_register_ic_object(&mut _azle_boa_context);
        _azle_unwrap_boa_result(
            _azle_boa_context.eval(format!(
                "let exports = {{}}; {compiled_js}",
                compiled_js = MAIN_JS
            )),
            &mut _azle_boa_context,
        );
        ic_cdk::timer::set_timer(core::time::Duration::new(0, 0), _azle_rng_seed);
    });
}
#[ic_cdk_macros::post_upgrade]
fn _azle_post_upgrade() {
    BOA_CONTEXT_REF_CELL.with(|box_context_ref_cell| {
        let mut _azle_boa_context = box_context_ref_cell.borrow_mut();
        METHOD_NAME_REF_CELL.with(|method_name_ref_cell| {
            let mut method_name_mut = method_name_ref_cell.borrow_mut();
            *method_name_mut = "DOES_NOT_EXIST".to_string()
        });
        _azle_register_ic_object(&mut _azle_boa_context);
        _azle_unwrap_boa_result(
            _azle_boa_context.eval(format!(
                "let exports = {{}}; {compiled_js}",
                compiled_js = MAIN_JS
            )),
            &mut _azle_boa_context,
        );
        ic_cdk::timer::set_timer(core::time::Duration::new(0, 0), _azle_rng_seed);
    });
}
#[ic_cdk_macros::pre_upgrade]
fn _azle_pre_upgrade() {
    BOA_CONTEXT_REF_CELL.with(|box_context_ref_cell| {
        let mut _azle_boa_context = box_context_ref_cell.borrow_mut();
        METHOD_NAME_REF_CELL.with(|method_name_ref_cell| {
            let mut method_name_mut = method_name_ref_cell.borrow_mut();
            *method_name_mut = "DOES_NOT_EXIST".to_string()
        });
    })
}
#[ic_cdk_macros::query()]
#[candid::candid_method(query)]
async fn hello_world() -> String {
    BOA_CONTEXT_REF_CELL.with(|box_context_ref_cell| {
        let mut _azle_boa_context = box_context_ref_cell.borrow_mut();
        let uuid = uuid::Uuid::new_v4().to_string();
        UUID_REF_CELL.with(|uuid_ref_cell| {
            let mut uuid_mut = uuid_ref_cell.borrow_mut();
            *uuid_mut = uuid.clone();
        });
        METHOD_NAME_REF_CELL.with(|method_name_ref_cell| {
            let mut method_name_mut = method_name_ref_cell.borrow_mut();
            *method_name_mut = "hello_world".to_string()
        });
        MANUAL_REF_CELL.with(|manual_ref_cell| {
            let mut manual_mut = manual_ref_cell.borrow_mut();
            *manual_mut = false;
        });
        let _azle_exports_js_value =
            _azle_unwrap_boa_result(_azle_boa_context.eval("exports"), &mut _azle_boa_context);
        let _azle_exports_js_object = _azle_exports_js_value.as_object().unwrap();
        let _azle_function_js_value = _azle_exports_js_object
            .get("hello_world", &mut _azle_boa_context)
            .unwrap();
        let _azle_function_js_object = _azle_function_js_value.as_object().unwrap();
        let _azle_boa_return_value = _azle_unwrap_boa_result(
            _azle_function_js_object.call(&boa_engine::JsValue::Null, &[], &mut _azle_boa_context),
            &mut _azle_boa_context,
        );
        let _azle_final_return_value = _azle_async_await_result_handler(
            &mut _azle_boa_context,
            &_azle_boa_return_value,
            &uuid,
            "hello_world",
            false,
        );
        match _azle_final_return_value.try_from_vm_value(&mut *_azle_boa_context) {
            Ok(return_value) => return_value,
            Err(e) => ic_cdk::api::trap(&format!("TypeError: {}", &e.0)),
        }
    })
}
#[doc = r" A marker type to match unconstrained callback arguments"]
#[derive(Debug, Clone, Copy, PartialEq, candid :: Deserialize)]
pub struct ArgToken;
impl candid::CandidType for ArgToken {
    fn _ty() -> candid::types::Type {
        candid::types::Type::Empty
    }
    fn idl_serialize<S: candid::types::Serializer>(&self, _serializer: S) -> Result<(), S::Error> {
        unimplemented!("Token is not serializable")
    }
}
fn _azle_async_await_result_handler(
    _azle_boa_context: &mut boa_engine::Context,
    _azle_boa_return_value: &boa_engine::JsValue,
    _azle_uuid: &str,
    _azle_method_name: &str,
    _azle_manual: bool,
) -> boa_engine::JsValue {
    if !_azle_boa_return_value.is_object()
        || !_azle_boa_return_value.as_object().unwrap().is_promise()
    {
        return _azle_boa_return_value.clone();
    }
    _azle_boa_context.eval("");
    let object = _azle_boa_return_value.as_object().unwrap().borrow();
    let promise = object.as_promise().unwrap();
    return match &promise.promise_state {
        boa_engine::builtins::promise::PromiseState::Fulfilled(js_value) => {
            PROMISE_MAP_REF_CELL.with(|promise_map_ref_cell| {
                let mut promise_map = promise_map_ref_cell.borrow_mut();
                promise_map.remove(_azle_uuid);
            });
            if _azle_manual == true {
                return _azle_boa_return_value.clone();
            }
            match _azle_method_name {
                "_AZLE_TIMER" => {}
                _ => panic!("method name was not found"),
            };
            return _azle_boa_return_value.clone();
        }
        boa_engine::builtins::promise::PromiseState::Rejected(js_value) => {
            PROMISE_MAP_REF_CELL.with(|promise_map_ref_cell| {
                let mut promise_map = promise_map_ref_cell.borrow_mut();
                promise_map.remove(_azle_uuid);
            });
            let error_message = _azle_js_value_to_string(js_value.clone(), _azle_boa_context);
            ic_cdk::api::trap(&format!("Uncaught {}", error_message));
        }
        boa_engine::builtins::promise::PromiseState::Pending => {
            PROMISE_MAP_REF_CELL.with(|promise_map_ref_cell| {
                let mut promise_map = promise_map_ref_cell.borrow_mut();
                promise_map.insert(_azle_uuid.to_string(), _azle_boa_return_value.clone());
            });
            return _azle_boa_return_value.clone();
        }
    };
}
pub fn _azle_unwrap_boa_result(
    boa_result: boa_engine::JsResult<boa_engine::JsValue>,
    context: &mut boa_engine::Context,
) -> boa_engine::JsValue {
    match boa_result {
        Ok(_azle_boa_return_value) => _azle_boa_return_value,
        Err(_azle_boa_error) => {
            let error_message =
                _azle_js_value_to_string(_azle_boa_error.to_opaque(context), context);
            ic_cdk::api::trap(&format!("Uncaught {}", error_message));
        }
    }
}
fn _azle_js_value_to_string(
    error_value: boa_engine::JsValue,
    context: &mut boa_engine::Context,
) -> String {
    match &error_value {
        boa_engine::JsValue::BigInt(bigint) => bigint.to_string(),
        boa_engine::JsValue::Boolean(boolean) => boolean.to_string(),
        boa_engine::JsValue::Integer(integer) => integer.to_string(),
        boa_engine::JsValue::Null => "null".to_string(),
        boa_engine::JsValue::Object(object) => {
            let to_string_js_value = object.get("toString", context).unwrap();
            let to_string_js_object = to_string_js_value.as_object().unwrap();
            let to_string_result = to_string_js_object.call(&error_value, &[], context);
            to_string_result
                .unwrap()
                .try_from_vm_value(context)
                .unwrap()
        }
        boa_engine::JsValue::Rational(rational) => rational.to_string(),
        boa_engine::JsValue::String(string) => string.to_std_string().unwrap(),
        boa_engine::JsValue::Symbol(symbol) => symbol.to_string(),
        boa_engine::JsValue::Undefined => "undefined".to_string(),
    }
}
fn _azle_ic_accept_message(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    Ok(ic_cdk::api::call::accept_message()
        .try_into_vm_value(_context)
        .unwrap())
}
fn _azle_ic_arg_data_raw(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    Ok(ic_cdk::api::call::arg_data_raw()
        .try_into_vm_value(_context)
        .unwrap())
}
fn _azle_ic_arg_data_raw_size(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    Ok(ic_cdk::api::call::arg_data_raw_size()
        .try_into_vm_value(_context)
        .unwrap())
}
fn _azle_ic_call_raw(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let promise_js_value = _context.eval("new Promise(() => {})").unwrap();
    let canister_id: ic_cdk::export::Principal = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let method: String = _aargs
        .get(1)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let args_raw: Vec<u8> = _aargs
        .get(2)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let payment: u64 = _aargs
        .get(3)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    _azle_ic_call_raw_spawn(&promise_js_value, canister_id, method, args_raw, payment);
    Ok(promise_js_value)
}
fn _azle_ic_call_raw_spawn(
    promise_js_value: &boa_engine::JsValue,
    canister_id: ic_cdk::export::Principal,
    method: String,
    args_raw: Vec<u8>,
    payment: u64,
) {
    let promise_js_value = promise_js_value.clone();
    ic_cdk::spawn(async move {
        let uuid = UUID_REF_CELL.with(|uuid_ref_cell| uuid_ref_cell.borrow().clone());
        let method_name =
            METHOD_NAME_REF_CELL.with(|method_name_ref_cell| method_name_ref_cell.borrow().clone());
        let manual = MANUAL_REF_CELL.with(|manual_ref_cell| manual_ref_cell.borrow().clone());
        let call_result =
            ic_cdk::api::call::call_raw(canister_id, &method, &args_raw, payment).await;
        UUID_REF_CELL.with(|uuid_ref_cell| {
            let mut uuid_mut = uuid_ref_cell.borrow_mut();
            *uuid_mut = uuid.clone();
        });
        METHOD_NAME_REF_CELL.with(|method_name_ref_cell| {
            let mut method_name_mut = method_name_ref_cell.borrow_mut();
            *method_name_mut = method_name.clone()
        });
        MANUAL_REF_CELL.with(|manual_ref_cell| {
            let mut manual_mut = manual_ref_cell.borrow_mut();
            *manual_mut = manual;
        });
        BOA_CONTEXT_REF_CELL.with(|box_context_ref_cell| {
            let mut _azle_boa_context = box_context_ref_cell.borrow_mut();
            let call_result_js_value = match call_result {
                Ok(value) => {
                    let js_value = value.try_into_vm_value(&mut *_azle_boa_context).unwrap();
                    let canister_result_js_object =
                        boa_engine::object::ObjectInitializer::new(&mut *_azle_boa_context)
                            .property("ok", js_value, boa_engine::property::Attribute::all())
                            .build();
                    let canister_result_js_value = canister_result_js_object.into();
                    canister_result_js_value
                }
                Err(err) => {
                    let js_value = format!(
                        "Rejection code {rejection_code}, {error_message}",
                        rejection_code = (err.0 as i32).to_string(),
                        error_message = err.1
                    )
                    .try_into_vm_value(&mut *_azle_boa_context)
                    .unwrap();
                    let canister_result_js_object =
                        boa_engine::object::ObjectInitializer::new(&mut *_azle_boa_context)
                            .property("err", js_value, boa_engine::property::Attribute::all())
                            .build();
                    let canister_result_js_value = canister_result_js_object.into();
                    canister_result_js_value
                }
            };
            let promise_js_object = promise_js_value.as_object().unwrap();
            let mut promise_object = promise_js_object.borrow_mut();
            let mut promise = promise_object.as_promise_mut().unwrap();
            promise.fulfill_promise(&call_result_js_value, &mut *_azle_boa_context);
            let main_promise = PROMISE_MAP_REF_CELL.with(|promise_map_ref_cell| {
                let promise_map = promise_map_ref_cell.borrow().clone();
                let main_promise = promise_map.get(&uuid).unwrap();
                main_promise.clone()
            });
            _azle_async_await_result_handler(
                &mut *_azle_boa_context,
                &main_promise,
                &uuid,
                &method_name,
                manual,
            );
        });
    });
}
fn _azle_ic_call_raw128(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let promise_js_value = _context.eval("new Promise(() => {})").unwrap();
    let canister_id: ic_cdk::export::Principal = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let method: String = _aargs
        .get(1)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let args_raw: Vec<u8> = _aargs
        .get(2)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let payment: u128 = _aargs
        .get(3)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    _azle_ic_call_raw128_spawn(&promise_js_value, canister_id, method, args_raw, payment);
    Ok(promise_js_value)
}
fn _azle_ic_call_raw128_spawn(
    promise_js_value: &boa_engine::JsValue,
    canister_id: ic_cdk::export::Principal,
    method: String,
    args_raw: Vec<u8>,
    payment: u128,
) {
    let promise_js_value = promise_js_value.clone();
    ic_cdk::spawn(async move {
        let uuid = UUID_REF_CELL.with(|uuid_ref_cell| uuid_ref_cell.borrow().clone());
        let method_name =
            METHOD_NAME_REF_CELL.with(|method_name_ref_cell| method_name_ref_cell.borrow().clone());
        let manual = MANUAL_REF_CELL.with(|manual_ref_cell| manual_ref_cell.borrow().clone());
        let call_result =
            ic_cdk::api::call::call_raw128(canister_id, &method, &args_raw, payment).await;
        UUID_REF_CELL.with(|uuid_ref_cell| {
            let mut uuid_mut = uuid_ref_cell.borrow_mut();
            *uuid_mut = uuid.clone();
        });
        METHOD_NAME_REF_CELL.with(|method_name_ref_cell| {
            let mut method_name_mut = method_name_ref_cell.borrow_mut();
            *method_name_mut = method_name.clone()
        });
        MANUAL_REF_CELL.with(|manual_ref_cell| {
            let mut manual_mut = manual_ref_cell.borrow_mut();
            *manual_mut = manual;
        });
        BOA_CONTEXT_REF_CELL.with(|box_context_ref_cell| {
            let mut _azle_boa_context = box_context_ref_cell.borrow_mut();
            let call_result_js_value = match call_result {
                Ok(value) => {
                    let js_value = value.try_into_vm_value(&mut *_azle_boa_context).unwrap();
                    let canister_result_js_object =
                        boa_engine::object::ObjectInitializer::new(&mut *_azle_boa_context)
                            .property("ok", js_value, boa_engine::property::Attribute::all())
                            .build();
                    let canister_result_js_value = canister_result_js_object.into();
                    canister_result_js_value
                }
                Err(err) => {
                    let js_value = format!(
                        "Rejection code {rejection_code}, {error_message}",
                        rejection_code = (err.0 as i32).to_string(),
                        error_message = err.1
                    )
                    .try_into_vm_value(&mut *_azle_boa_context)
                    .unwrap();
                    let canister_result_js_object =
                        boa_engine::object::ObjectInitializer::new(&mut *_azle_boa_context)
                            .property("err", js_value, boa_engine::property::Attribute::all())
                            .build();
                    let canister_result_js_value = canister_result_js_object.into();
                    canister_result_js_value
                }
            };
            let promise_js_object = promise_js_value.as_object().unwrap();
            let mut promise_object = promise_js_object.borrow_mut();
            let mut promise = promise_object.as_promise_mut().unwrap();
            promise.fulfill_promise(&call_result_js_value, &mut *_azle_boa_context);
            let main_promise = PROMISE_MAP_REF_CELL.with(|promise_map_ref_cell| {
                let promise_map = promise_map_ref_cell.borrow().clone();
                let main_promise = promise_map.get(&uuid).unwrap();
                main_promise.clone()
            });
            _azle_async_await_result_handler(
                &mut *_azle_boa_context,
                &main_promise,
                &uuid,
                &method_name,
                manual,
            );
        });
    });
}
fn _azle_ic_caller(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    Ok(ic_cdk::api::caller().try_into_vm_value(_context).unwrap())
}
fn _azle_ic_candid_decode(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let candid_encoded: Vec<u8> = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let candid_args: candid::IDLArgs = candid::IDLArgs::from_bytes(&candid_encoded).unwrap();
    let candid_string = candid_args.to_string();
    Ok(candid_string.try_into_vm_value(_context).unwrap())
}
fn _azle_ic_candid_encode(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let candid_string: String = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let candid_args: candid::IDLArgs = candid_string.parse().unwrap();
    let candid_encoded: Vec<u8> = candid_args.to_bytes().unwrap();
    Ok(candid_encoded.try_into_vm_value(&mut *_context).unwrap())
}
fn _azle_ic_canister_balance(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    Ok(ic_cdk::api::canister_balance()
        .try_into_vm_value(_context)
        .unwrap())
}
fn _azle_ic_canister_balance128(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    Ok(ic_cdk::api::canister_balance128()
        .try_into_vm_value(_context)
        .unwrap())
}
fn _azle_ic_clear_timer(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let timer_id_js_value = _aargs.get(0).unwrap().clone();
    let timer_id: ic_cdk::timer::TimerId =
        timer_id_js_value.try_from_vm_value(&mut *_context).unwrap();
    ic_cdk::timer::clear_timer(timer_id);
    Ok(boa_engine::JsValue::Undefined)
}
fn _azle_ic_data_certificate(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    Ok(ic_cdk::api::data_certificate()
        .try_into_vm_value(_context)
        .unwrap())
}
fn _azle_ic_id(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    Ok(ic_cdk::api::id().try_into_vm_value(_context).unwrap())
}
fn _azle_ic_method_name(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    Ok(ic_cdk::api::call::method_name().into())
}
fn _azle_ic_msg_cycles_accept(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let max_amount: u64 = _aargs[0].clone().try_from_vm_value(_context).unwrap();
    let return_value: boa_engine::bigint::JsBigInt =
        ic_cdk::api::call::msg_cycles_accept(max_amount).into();
    Ok(return_value.into())
}
fn _azle_ic_msg_cycles_accept128(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let max_amount: u128 = _aargs[0].clone().try_from_vm_value(_context).unwrap();
    let return_value: boa_engine::bigint::JsBigInt =
        ic_cdk::api::call::msg_cycles_accept128(max_amount).into();
    Ok(return_value.into())
}
fn _azle_ic_msg_cycles_available(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let return_value: boa_engine::bigint::JsBigInt =
        ic_cdk::api::call::msg_cycles_available().into();
    Ok(return_value.into())
}
fn _azle_ic_msg_cycles_available128(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let return_value: boa_engine::bigint::JsBigInt =
        ic_cdk::api::call::msg_cycles_available().into();
    Ok(return_value.into())
}
fn _azle_ic_msg_cycles_refunded(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let return_value: boa_engine::bigint::JsBigInt =
        ic_cdk::api::call::msg_cycles_refunded().into();
    Ok(return_value.into())
}
fn _azle_ic_msg_cycles_refunded128(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let return_value: boa_engine::bigint::JsBigInt =
        ic_cdk::api::call::msg_cycles_refunded128().into();
    Ok(return_value.into())
}
fn _azle_ic_notify_raw(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let canister_id_js_value = _aargs.get(0).unwrap().clone();
    let canister_id_principal: ic_cdk::export::Principal = canister_id_js_value
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let method_js_value = _aargs.get(1).unwrap().clone();
    let method_string: String = method_js_value.try_from_vm_value(&mut *_context).unwrap();
    let args_raw_js_value = _aargs.get(2).unwrap().clone();
    let args_raw_vec: Vec<u8> = args_raw_js_value.try_from_vm_value(&mut *_context).unwrap();
    let payment_js_value = _aargs.get(3).unwrap().clone();
    let payment: u128 = payment_js_value.try_from_vm_value(&mut *_context).unwrap();
    let notify_result = ic_cdk::api::call::notify_raw(
        canister_id_principal,
        &method_string,
        &args_raw_vec,
        payment,
    );
    Ok(notify_result.try_into_vm_value(_context).unwrap())
}
fn _azle_ic_performance_counter(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let counter_type: u32 = _aargs[0].clone().try_from_vm_value(_context).unwrap();
    let return_value: boa_engine::bigint::JsBigInt =
        ic_cdk::api::call::performance_counter(counter_type).into();
    Ok(return_value.into())
}
fn _azle_ic_print(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    ic_cdk::println!("{:#?}", _aargs);
    return Ok(boa_engine::JsValue::Undefined);
}
fn _azle_ic_reject(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let message: String = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    Ok(ic_cdk::api::call::reject(&message)
        .try_into_vm_value(&mut *_context)
        .unwrap())
}
fn _azle_ic_reject_code(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    Ok(ic_cdk::api::call::reject_code()
        .try_into_vm_value(_context)
        .unwrap())
}
fn _azle_ic_reject_message(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    Ok(ic_cdk::api::call::reject_message()
        .try_into_vm_value(_context)
        .unwrap())
}
fn _azle_ic_reply(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let method_name =
        METHOD_NAME_REF_CELL.with(|method_name_ref_cell| method_name_ref_cell.borrow().clone());
    match &method_name[..] {
        _ => panic!("This cannot happen"),
    }
}
fn _azle_ic_reply_raw(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let buf_js_value: boa_engine::JsValue = _aargs.get(0).unwrap().clone();
    let buf_vec: Vec<u8> = buf_js_value.try_from_vm_value(&mut *_context).unwrap();
    Ok(ic_cdk::api::call::reply_raw(&buf_vec)
        .try_into_vm_value(_context)
        .unwrap())
}
fn _azle_ic_set_certified_data(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let data_js_value: boa_engine::JsValue = _aargs.get(0).unwrap().clone();
    let data_vec: Vec<u8> = data_js_value.try_from_vm_value(&mut *_context).unwrap();
    Ok(ic_cdk::api::set_certified_data(&data_vec)
        .try_into_vm_value(_context)
        .unwrap())
}
fn _azle_ic_set_timer(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let delay_js_value = _aargs.get(0).unwrap().clone();
    let delay_as_u64: u64 = delay_js_value.try_from_vm_value(&mut *_context).unwrap();
    let delay = core::time::Duration::new(delay_as_u64, 0);
    let func_js_value = _aargs.get(1).unwrap();
    let func_js_object = func_js_value.as_object().unwrap().clone();
    let closure = move || {
        BOA_CONTEXT_REF_CELL.with(|boa_context_ref_cell| {
            let mut _azle_boa_context = boa_context_ref_cell.borrow_mut();
            let uuid = uuid::Uuid::new_v4().to_string();
            UUID_REF_CELL.with(|uuid_ref_cell| {
                let mut uuid_mut = uuid_ref_cell.borrow_mut();
                *uuid_mut = uuid.clone();
            });
            METHOD_NAME_REF_CELL.with(|method_name_ref_cell| {
                let mut method_name_mut = method_name_ref_cell.borrow_mut();
                *method_name_mut = "_AZLE_TIMER".to_string();
            });
            MANUAL_REF_CELL.with(|manual_ref_cell| {
                let mut manual_mut = manual_ref_cell.borrow_mut();
                *manual_mut = false;
            });
            let _azle_boa_return_value = _azle_unwrap_boa_result(
                func_js_object.call(&boa_engine::JsValue::Null, &[], &mut *_azle_boa_context),
                &mut *_azle_boa_context,
            );
            _azle_async_await_result_handler(
                &mut _azle_boa_context,
                &_azle_boa_return_value,
                &uuid,
                "_AZLE_TIMER",
                false,
            );
        });
    };
    let timer_id = ic_cdk::timer::set_timer(delay, closure);
    Ok(timer_id.try_into_vm_value(_context).unwrap())
}
fn _azle_ic_set_timer_interval(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let interval_js_value = _aargs.get(0).unwrap().clone();
    let interval_as_u64: u64 = interval_js_value.try_from_vm_value(&mut *_context).unwrap();
    let interval = core::time::Duration::new(interval_as_u64, 0);
    let func_js_value = _aargs.get(1).unwrap();
    let func_js_object = func_js_value.as_object().unwrap().clone();
    let closure = move || {
        BOA_CONTEXT_REF_CELL.with(|boa_context_ref_cell| {
            let mut _azle_boa_context = boa_context_ref_cell.borrow_mut();
            let uuid = uuid::Uuid::new_v4().to_string();
            UUID_REF_CELL.with(|uuid_ref_cell| {
                let mut uuid_mut = uuid_ref_cell.borrow_mut();
                *uuid_mut = uuid.clone();
            });
            METHOD_NAME_REF_CELL.with(|method_name_ref_cell| {
                let mut method_name_mut = method_name_ref_cell.borrow_mut();
                *method_name_mut = "_AZLE_TIMER".to_string();
            });
            MANUAL_REF_CELL.with(|manual_ref_cell| {
                let mut manual_mut = manual_ref_cell.borrow_mut();
                *manual_mut = false;
            });
            let _azle_boa_return_value = _azle_unwrap_boa_result(
                func_js_object.call(&boa_engine::JsValue::Null, &[], &mut *_azle_boa_context),
                &mut *_azle_boa_context,
            );
            _azle_async_await_result_handler(
                &mut _azle_boa_context,
                &_azle_boa_return_value,
                &uuid,
                "_AZLE_TIMER",
                false,
            );
        });
    };
    let timer_id = ic_cdk::timer::set_timer_interval(interval, closure);
    Ok(timer_id.try_into_vm_value(_context).unwrap())
}
fn _azle_ic_stable_b_tree_map_contains_key(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let memory_id: u8 = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let key_js_value = _aargs.get(1).unwrap().clone();
    match memory_id {
        _ => panic!(
            "memory_id {} does not have an associated StableBTreeMap",
            memory_id
        ),
    }
}
fn _azle_ic_stable_b_tree_map_get(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let memory_id: u8 = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let key_js_value = _aargs.get(1).unwrap().clone();
    match memory_id {
        _ => panic!(
            "memory_id {} does not have an associated StableBTreeMap",
            memory_id
        ),
    }
}
fn _azle_ic_stable_b_tree_map_insert(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let memory_id: u8 = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let key_js_value = _aargs.get(1).unwrap().clone();
    let value_js_value = _aargs.get(2).unwrap().clone();
    match memory_id {
        _ => panic!(
            "memory_id {} does not have an associated StableBTreeMap",
            memory_id
        ),
    }
}
fn _azle_ic_stable_b_tree_map_is_empty(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let memory_id: u8 = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    match memory_id {
        _ => panic!(
            "memory_id {} does not have an associated StableBTreeMap",
            memory_id
        ),
    }
}
fn _azle_ic_stable_b_tree_map_items(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let memory_id: u8 = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    match memory_id {
        _ => panic!(
            "memory_id {} does not have an associated StableBTreeMap",
            memory_id
        ),
    }
}
fn _azle_ic_stable_b_tree_map_keys(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let memory_id: u8 = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    match memory_id {
        _ => panic!(
            "memory_id {} does not have an associated StableBTreeMap",
            memory_id
        ),
    }
}
fn _azle_ic_stable_b_tree_map_len(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let memory_id: u8 = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    match memory_id {
        _ => panic!(
            "memory_id {} does not have an associated StableBTreeMap",
            memory_id
        ),
    }
}
fn _azle_ic_stable_b_tree_map_remove(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let memory_id: u8 = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let key_js_value = _aargs.get(1).unwrap().clone();
    match memory_id {
        _ => panic!(
            "memory_id {} does not have an associated StableBTreeMap",
            memory_id
        ),
    }
}
fn _azle_ic_stable_b_tree_map_values(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let memory_id: u8 = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    match memory_id {
        _ => panic!(
            "memory_id {} does not have an associated StableBTreeMap",
            memory_id
        ),
    }
}
fn _azle_ic_stable64_grow(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let new_pages: u64 = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    Ok(ic_cdk::api::stable::stable64_grow(new_pages)
        .try_into_vm_value(_context)
        .unwrap())
}
fn _azle_ic_stable64_read(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let offset: u64 = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let length: u64 = _aargs
        .get(1)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let mut buf: Vec<u8> = vec![0; length as usize];
    ic_cdk::api::stable::stable64_read(offset, &mut buf);
    Ok(buf.to_vec().try_into_vm_value(_context).unwrap())
}
fn _azle_ic_stable64_size(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    Ok(ic_cdk::api::stable::stable64_size()
        .try_into_vm_value(_context)
        .unwrap())
}
fn _azle_ic_stable64_write(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let offset: u64 = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let buf_vector: Vec<u8> = _aargs
        .get(1)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let buf: &[u8] = &buf_vector[..];
    ic_cdk::api::stable::stable64_write(offset, buf);
    Ok(boa_engine::JsValue::Undefined)
}
fn _azle_ic_stable_bytes(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    Ok(ic_cdk::api::stable::stable_bytes()
        .try_into_vm_value(_context)
        .unwrap())
}
fn _azle_ic_stable_grow(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let new_pages: u32 = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    Ok(ic_cdk::api::stable::stable_grow(new_pages)
        .try_into_vm_value(_context)
        .unwrap())
}
fn _azle_ic_stable_read(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let offset: u32 = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let length: u32 = _aargs
        .get(1)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let mut buf: Vec<u8> = vec![0; length as usize];
    ic_cdk::api::stable::stable_read(offset, &mut buf);
    Ok(buf.try_into_vm_value(_context).unwrap())
}
fn _azle_ic_stable_size(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    Ok(ic_cdk::api::stable::stable_size()
        .try_into_vm_value(_context)
        .unwrap())
}
fn _azle_ic_stable_write(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let offset: u32 = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let buf_vector: Vec<u8> = _aargs
        .get(1)
        .unwrap()
        .clone()
        .try_from_vm_value(&mut *_context)
        .unwrap();
    let buf: &[u8] = &buf_vector[..];
    ic_cdk::api::stable::stable_write(offset, buf);
    Ok(boa_engine::JsValue::Undefined)
}
fn _azle_ic_time(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    Ok(ic_cdk::api::time().try_into_vm_value(_context).unwrap())
}
fn _azle_ic_trap(
    _this: &boa_engine::JsValue,
    _aargs: &[boa_engine::JsValue],
    _context: &mut boa_engine::Context,
) -> boa_engine::JsResult<boa_engine::JsValue> {
    let message: String = _aargs
        .get(0)
        .unwrap()
        .clone()
        .try_from_vm_value(_context)
        .unwrap();
    ic_cdk::api::trap(&message);
}
fn _azle_register_ic_object(boa_context: &mut boa_engine::Context) {
    let ic = boa_engine::object::ObjectInitializer::new(boa_context)
        .function(_azle_ic_accept_message, "accept_message", 0)
        .function(_azle_ic_arg_data_raw, "arg_data_raw", 0)
        .function(_azle_ic_arg_data_raw_size, "arg_data_raw_size", 0)
        .function(_azle_ic_call_raw, "call_raw", 0)
        .function(_azle_ic_call_raw128, "call_raw128", 0)
        .function(_azle_ic_caller, "caller", 0)
        .function(_azle_ic_candid_decode, "candid_decode", 0)
        .function(_azle_ic_candid_encode, "candid_encode", 0)
        .function(_azle_ic_canister_balance, "canister_balance", 0)
        .function(_azle_ic_canister_balance128, "canister_balance128", 0)
        .function(_azle_ic_clear_timer, "clear_timer", 0)
        .function(_azle_ic_data_certificate, "data_certificate", 0)
        .function(_azle_ic_id, "id", 0)
        .function(_azle_ic_method_name, "method_name", 0)
        .function(_azle_ic_msg_cycles_accept, "msg_cycles_accept", 0)
        .function(_azle_ic_msg_cycles_accept128, "msg_cycles_accept128", 0)
        .function(_azle_ic_msg_cycles_available, "msg_cycles_available", 0)
        .function(
            _azle_ic_msg_cycles_available128,
            "msg_cycles_available128",
            0,
        )
        .function(_azle_ic_msg_cycles_refunded, "msg_cycles_refunded", 0)
        .function(_azle_ic_msg_cycles_refunded128, "msg_cycles_refunded128", 0)
        .function(_azle_ic_notify_raw, "notify_raw", 0)
        .function(_azle_ic_performance_counter, "performance_counter", 0)
        .function(_azle_ic_print, "print", 0)
        .function(_azle_ic_reject, "reject", 0)
        .function(_azle_ic_reject_code, "reject_code", 0)
        .function(_azle_ic_reject_message, "reject_message", 0)
        .function(_azle_ic_reply, "reply", 0)
        .function(_azle_ic_reply_raw, "reply_raw", 0)
        .function(_azle_ic_set_certified_data, "set_certified_data", 0)
        .function(_azle_ic_set_timer, "set_timer", 0)
        .function(_azle_ic_set_timer_interval, "set_timer_interval", 0)
        .function(_azle_ic_stable_bytes, "stable_bytes", 0)
        .function(
            _azle_ic_stable_b_tree_map_contains_key,
            "stable_b_tree_map_contains_key",
            0,
        )
        .function(_azle_ic_stable_b_tree_map_get, "stable_b_tree_map_get", 0)
        .function(
            _azle_ic_stable_b_tree_map_insert,
            "stable_b_tree_map_insert",
            0,
        )
        .function(
            _azle_ic_stable_b_tree_map_is_empty,
            "stable_b_tree_map_is_empty",
            0,
        )
        .function(
            _azle_ic_stable_b_tree_map_items,
            "stable_b_tree_map_items",
            0,
        )
        .function(_azle_ic_stable_b_tree_map_keys, "stable_b_tree_map_keys", 0)
        .function(
            _azle_ic_stable_b_tree_map_values,
            "stable_b_tree_map_values",
            0,
        )
        .function(_azle_ic_stable_b_tree_map_len, "stable_b_tree_map_len", 0)
        .function(
            _azle_ic_stable_b_tree_map_remove,
            "stable_b_tree_map_remove",
            0,
        )
        .function(_azle_ic_stable_grow, "stable_grow", 0)
        .function(_azle_ic_stable_read, "stable_read", 0)
        .function(_azle_ic_stable_size, "stable_size", 0)
        .function(_azle_ic_stable_write, "stable_write", 0)
        .function(_azle_ic_stable64_grow, "stable64_grow", 0)
        .function(_azle_ic_stable64_read, "stable64_read", 0)
        .function(_azle_ic_stable64_size, "stable64_size", 0)
        .function(_azle_ic_stable64_write, "stable64_write", 0)
        .function(_azle_ic_time, "time", 0)
        .function(_azle_ic_trap, "trap", 0)
        .build();
    boa_context.register_global_property("ic", ic, boa_engine::property::Attribute::all());
}
use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
type AzleMemory = VirtualMemory<DefaultMemoryImpl>;
thread_local! { static MEMORY_MANAGER_REF_CELL : RefCell < MemoryManager < DefaultMemoryImpl >> = RefCell :: new (MemoryManager :: init (DefaultMemoryImpl :: default ())) ; }
fn _azle_rng_seed() {
    ic_cdk::spawn(async move {
        let result: CallResult<(Vec<u8>,)> = ic_cdk::api::call::call(
            candid::Principal::from_text("aaaaa-aa").unwrap(),
            "raw_rand",
            (),
        )
        .await;
        RNG_REF_CELL.with(|rng_ref_cell| {
            let mut rng = rng_ref_cell.borrow_mut();
            match result {
                Ok(randomness) => {
                    *rng = SeedableRng::from_seed(randomness.0[..].try_into().unwrap())
                }
                Err(err) => panic!(err),
            };
        });
    });
}
candid::export_service!();
#[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
fn _azle_export_candid() -> String {
    __export_service()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn _azle_write_candid_to_disk() {
        std::fs::write("index.did", _azle_export_candid()).unwrap();
    }
}
