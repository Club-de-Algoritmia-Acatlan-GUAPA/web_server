(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[541],{3454:function(t,r,e){"use strict";var o,i;t.exports=(null==(o=e.g.process)?void 0:o.env)&&"object"==typeof(null==(i=e.g.process)?void 0:i.env)?e.g.process:e(7663)},1876:function(t){!function(){var r={675:function(t,r){"use strict";r.byteLength=function(t){var r=a(t),e=r[0],o=r[1];return(e+o)*3/4-o},r.toByteArray=function(t){var r,e,f=a(t),u=f[0],s=f[1],h=new i((u+s)*3/4-s),c=0,l=s>0?u-4:u;for(e=0;e<l;e+=4)r=o[t.charCodeAt(e)]<<18|o[t.charCodeAt(e+1)]<<12|o[t.charCodeAt(e+2)]<<6|o[t.charCodeAt(e+3)],h[c++]=r>>16&255,h[c++]=r>>8&255,h[c++]=255&r;return 2===s&&(r=o[t.charCodeAt(e)]<<2|o[t.charCodeAt(e+1)]>>4,h[c++]=255&r),1===s&&(r=o[t.charCodeAt(e)]<<10|o[t.charCodeAt(e+1)]<<4|o[t.charCodeAt(e+2)]>>2,h[c++]=r>>8&255,h[c++]=255&r),h},r.fromByteArray=function(t){for(var r,o=t.length,i=o%3,f=[],u=0,s=o-i;u<s;u+=16383)f.push(function(t,r,o){for(var i,f=[],u=r;u<o;u+=3)f.push(e[(i=(t[u]<<16&16711680)+(t[u+1]<<8&65280)+(255&t[u+2]))>>18&63]+e[i>>12&63]+e[i>>6&63]+e[63&i]);return f.join("")}(t,u,u+16383>s?s:u+16383));return 1===i?f.push(e[(r=t[o-1])>>2]+e[r<<4&63]+"=="):2===i&&f.push(e[(r=(t[o-2]<<8)+t[o-1])>>10]+e[r>>4&63]+e[r<<2&63]+"="),f.join("")};for(var e=[],o=[],i="undefined"!=typeof Uint8Array?Uint8Array:Array,f="ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",u=0,s=f.length;u<s;++u)e[u]=f[u],o[f.charCodeAt(u)]=u;function a(t){var r=t.length;if(r%4>0)throw Error("Invalid string. Length must be a multiple of 4");var e=t.indexOf("=");-1===e&&(e=r);var o=e===r?0:4-e%4;return[e,o]}o["-".charCodeAt(0)]=62,o["_".charCodeAt(0)]=63},72:function(t,r,e){"use strict";/*!
 * The buffer module from node.js, for the browser.
 *
 * @author   Feross Aboukhadijeh <https://feross.org>
 * @license  MIT
 */var o=e(675),i=e(783),f="function"==typeof Symbol&&"function"==typeof Symbol.for?Symbol.for("nodejs.util.inspect.custom"):null;function u(t){if(t>2147483647)throw RangeError('The value "'+t+'" is invalid for option "size"');var r=new Uint8Array(t);return Object.setPrototypeOf(r,s.prototype),r}function s(t,r,e){if("number"==typeof t){if("string"==typeof r)throw TypeError('The "string" argument must be of type string. Received type number');return c(t)}return a(t,r,e)}function a(t,r,e){if("string"==typeof t)return function(t,r){if(("string"!=typeof r||""===r)&&(r="utf8"),!s.isEncoding(r))throw TypeError("Unknown encoding: "+r);var e=0|y(t,r),o=u(e),i=o.write(t,r);return i!==e&&(o=o.slice(0,i)),o}(t,r);if(ArrayBuffer.isView(t))return l(t);if(null==t)throw TypeError("The first argument must be one of type string, Buffer, ArrayBuffer, Array, or Array-like Object. Received type "+typeof t);if(I(t,ArrayBuffer)||t&&I(t.buffer,ArrayBuffer)||"undefined"!=typeof SharedArrayBuffer&&(I(t,SharedArrayBuffer)||t&&I(t.buffer,SharedArrayBuffer)))return function(t,r,e){var o;if(r<0||t.byteLength<r)throw RangeError('"offset" is outside of buffer bounds');if(t.byteLength<r+(e||0))throw RangeError('"length" is outside of buffer bounds');return Object.setPrototypeOf(o=void 0===r&&void 0===e?new Uint8Array(t):void 0===e?new Uint8Array(t,r):new Uint8Array(t,r,e),s.prototype),o}(t,r,e);if("number"==typeof t)throw TypeError('The "value" argument must not be of type number. Received type number');var o=t.valueOf&&t.valueOf();if(null!=o&&o!==t)return s.from(o,r,e);var i=function(t){if(s.isBuffer(t)){var r,e=0|p(t.length),o=u(e);return 0===o.length||t.copy(o,0,0,e),o}return void 0!==t.length?"number"!=typeof t.length||(r=t.length)!=r?u(0):l(t):"Buffer"===t.type&&Array.isArray(t.data)?l(t.data):void 0}(t);if(i)return i;if("undefined"!=typeof Symbol&&null!=Symbol.toPrimitive&&"function"==typeof t[Symbol.toPrimitive])return s.from(t[Symbol.toPrimitive]("string"),r,e);throw TypeError("The first argument must be one of type string, Buffer, ArrayBuffer, Array, or Array-like Object. Received type "+typeof t)}function h(t){if("number"!=typeof t)throw TypeError('"size" argument must be of type number');if(t<0)throw RangeError('The value "'+t+'" is invalid for option "size"')}function c(t){return h(t),u(t<0?0:0|p(t))}function l(t){for(var r=t.length<0?0:0|p(t.length),e=u(r),o=0;o<r;o+=1)e[o]=255&t[o];return e}function p(t){if(t>=2147483647)throw RangeError("Attempt to allocate Buffer larger than maximum size: 0x7fffffff bytes");return 0|t}function y(t,r){if(s.isBuffer(t))return t.length;if(ArrayBuffer.isView(t)||I(t,ArrayBuffer))return t.byteLength;if("string"!=typeof t)throw TypeError('The "string" argument must be one of type string, Buffer, or ArrayBuffer. Received type '+typeof t);var e=t.length,o=arguments.length>2&&!0===arguments[2];if(!o&&0===e)return 0;for(var i=!1;;)switch(r){case"ascii":case"latin1":case"binary":return e;case"utf8":case"utf-8":return U(t).length;case"ucs2":case"ucs-2":case"utf16le":case"utf-16le":return 2*e;case"hex":return e>>>1;case"base64":return x(t).length;default:if(i)return o?-1:U(t).length;r=(""+r).toLowerCase(),i=!0}}function g(t,r,e){var i,f,u=!1;if((void 0===r||r<0)&&(r=0),r>this.length||((void 0===e||e>this.length)&&(e=this.length),e<=0||(e>>>=0)<=(r>>>=0)))return"";for(t||(t="utf8");;)switch(t){case"hex":return function(t,r,e){var o=t.length;(!r||r<0)&&(r=0),(!e||e<0||e>o)&&(e=o);for(var i="",f=r;f<e;++f)i+=R[t[f]];return i}(this,r,e);case"utf8":case"utf-8":return m(this,r,e);case"ascii":return function(t,r,e){var o="";e=Math.min(t.length,e);for(var i=r;i<e;++i)o+=String.fromCharCode(127&t[i]);return o}(this,r,e);case"latin1":case"binary":return function(t,r,e){var o="";e=Math.min(t.length,e);for(var i=r;i<e;++i)o+=String.fromCharCode(t[i]);return o}(this,r,e);case"base64":return i=r,f=e,0===i&&f===this.length?o.fromByteArray(this):o.fromByteArray(this.slice(i,f));case"ucs2":case"ucs-2":case"utf16le":case"utf-16le":return function(t,r,e){for(var o=t.slice(r,e),i="",f=0;f<o.length;f+=2)i+=String.fromCharCode(o[f]+256*o[f+1]);return i}(this,r,e);default:if(u)throw TypeError("Unknown encoding: "+t);t=(t+"").toLowerCase(),u=!0}}function d(t,r,e){var o=t[r];t[r]=t[e],t[e]=o}function v(t,r,e,o,i){var f;if(0===t.length)return -1;if("string"==typeof e?(o=e,e=0):e>2147483647?e=2147483647:e<-2147483648&&(e=-2147483648),(f=e=+e)!=f&&(e=i?0:t.length-1),e<0&&(e=t.length+e),e>=t.length){if(i)return -1;e=t.length-1}else if(e<0){if(!i)return -1;e=0}if("string"==typeof r&&(r=s.from(r,o)),s.isBuffer(r))return 0===r.length?-1:b(t,r,e,o,i);if("number"==typeof r)return(r&=255,"function"==typeof Uint8Array.prototype.indexOf)?i?Uint8Array.prototype.indexOf.call(t,r,e):Uint8Array.prototype.lastIndexOf.call(t,r,e):b(t,[r],e,o,i);throw TypeError("val must be string, number or Buffer")}function b(t,r,e,o,i){var f,u=1,s=t.length,a=r.length;if(void 0!==o&&("ucs2"===(o=String(o).toLowerCase())||"ucs-2"===o||"utf16le"===o||"utf-16le"===o)){if(t.length<2||r.length<2)return -1;u=2,s/=2,a/=2,e/=2}function h(t,r){return 1===u?t[r]:t.readUInt16BE(r*u)}if(i){var c=-1;for(f=e;f<s;f++)if(h(t,f)===h(r,-1===c?0:f-c)){if(-1===c&&(c=f),f-c+1===a)return c*u}else -1!==c&&(f-=f-c),c=-1}else for(e+a>s&&(e=s-a),f=e;f>=0;f--){for(var l=!0,p=0;p<a;p++)if(h(t,f+p)!==h(r,p)){l=!1;break}if(l)return f}return -1}function m(t,r,e){e=Math.min(t.length,e);for(var o=[],i=r;i<e;){var f,u,s,a,h=t[i],c=null,l=h>239?4:h>223?3:h>191?2:1;if(i+l<=e)switch(l){case 1:h<128&&(c=h);break;case 2:(192&(f=t[i+1]))==128&&(a=(31&h)<<6|63&f)>127&&(c=a);break;case 3:f=t[i+1],u=t[i+2],(192&f)==128&&(192&u)==128&&(a=(15&h)<<12|(63&f)<<6|63&u)>2047&&(a<55296||a>57343)&&(c=a);break;case 4:f=t[i+1],u=t[i+2],s=t[i+3],(192&f)==128&&(192&u)==128&&(192&s)==128&&(a=(15&h)<<18|(63&f)<<12|(63&u)<<6|63&s)>65535&&a<1114112&&(c=a)}null===c?(c=65533,l=1):c>65535&&(c-=65536,o.push(c>>>10&1023|55296),c=56320|1023&c),o.push(c),i+=l}return function(t){var r=t.length;if(r<=4096)return String.fromCharCode.apply(String,t);for(var e="",o=0;o<r;)e+=String.fromCharCode.apply(String,t.slice(o,o+=4096));return e}(o)}function w(t,r,e){if(t%1!=0||t<0)throw RangeError("offset is not uint");if(t+r>e)throw RangeError("Trying to access beyond buffer length")}function E(t,r,e,o,i,f){if(!s.isBuffer(t))throw TypeError('"buffer" argument must be a Buffer instance');if(r>i||r<f)throw RangeError('"value" argument is out of bounds');if(e+o>t.length)throw RangeError("Index out of range")}function A(t,r,e,o,i,f){if(e+o>t.length||e<0)throw RangeError("Index out of range")}function B(t,r,e,o,f){return r=+r,e>>>=0,f||A(t,r,e,4,34028234663852886e22,-34028234663852886e22),i.write(t,r,e,o,23,4),e+4}function T(t,r,e,o,f){return r=+r,e>>>=0,f||A(t,r,e,8,17976931348623157e292,-17976931348623157e292),i.write(t,r,e,o,52,8),e+8}r.Buffer=s,r.SlowBuffer=function(t){return+t!=t&&(t=0),s.alloc(+t)},r.INSPECT_MAX_BYTES=50,r.kMaxLength=2147483647,s.TYPED_ARRAY_SUPPORT=function(){try{var t=new Uint8Array(1),r={foo:function(){return 42}};return Object.setPrototypeOf(r,Uint8Array.prototype),Object.setPrototypeOf(t,r),42===t.foo()}catch(e){return!1}}(),s.TYPED_ARRAY_SUPPORT||"undefined"==typeof console||"function"!=typeof console.error||console.error("This browser lacks typed array (Uint8Array) support which is required by `buffer` v5.x. Use `buffer` v4.x if you require old browser support."),Object.defineProperty(s.prototype,"parent",{enumerable:!0,get:function(){if(s.isBuffer(this))return this.buffer}}),Object.defineProperty(s.prototype,"offset",{enumerable:!0,get:function(){if(s.isBuffer(this))return this.byteOffset}}),s.poolSize=8192,s.from=function(t,r,e){return a(t,r,e)},Object.setPrototypeOf(s.prototype,Uint8Array.prototype),Object.setPrototypeOf(s,Uint8Array),s.alloc=function(t,r,e){return(h(t),t<=0)?u(t):void 0!==r?"string"==typeof e?u(t).fill(r,e):u(t).fill(r):u(t)},s.allocUnsafe=function(t){return c(t)},s.allocUnsafeSlow=function(t){return c(t)},s.isBuffer=function(t){return null!=t&&!0===t._isBuffer&&t!==s.prototype},s.compare=function(t,r){if(I(t,Uint8Array)&&(t=s.from(t,t.offset,t.byteLength)),I(r,Uint8Array)&&(r=s.from(r,r.offset,r.byteLength)),!s.isBuffer(t)||!s.isBuffer(r))throw TypeError('The "buf1", "buf2" arguments must be one of type Buffer or Uint8Array');if(t===r)return 0;for(var e=t.length,o=r.length,i=0,f=Math.min(e,o);i<f;++i)if(t[i]!==r[i]){e=t[i],o=r[i];break}return e<o?-1:o<e?1:0},s.isEncoding=function(t){switch(String(t).toLowerCase()){case"hex":case"utf8":case"utf-8":case"ascii":case"latin1":case"binary":case"base64":case"ucs2":case"ucs-2":case"utf16le":case"utf-16le":return!0;default:return!1}},s.concat=function(t,r){if(!Array.isArray(t))throw TypeError('"list" argument must be an Array of Buffers');if(0===t.length)return s.alloc(0);if(void 0===r)for(e=0,r=0;e<t.length;++e)r+=t[e].length;var e,o=s.allocUnsafe(r),i=0;for(e=0;e<t.length;++e){var f=t[e];if(I(f,Uint8Array)&&(f=s.from(f)),!s.isBuffer(f))throw TypeError('"list" argument must be an Array of Buffers');f.copy(o,i),i+=f.length}return o},s.byteLength=y,s.prototype._isBuffer=!0,s.prototype.swap16=function(){var t=this.length;if(t%2!=0)throw RangeError("Buffer size must be a multiple of 16-bits");for(var r=0;r<t;r+=2)d(this,r,r+1);return this},s.prototype.swap32=function(){var t=this.length;if(t%4!=0)throw RangeError("Buffer size must be a multiple of 32-bits");for(var r=0;r<t;r+=4)d(this,r,r+3),d(this,r+1,r+2);return this},s.prototype.swap64=function(){var t=this.length;if(t%8!=0)throw RangeError("Buffer size must be a multiple of 64-bits");for(var r=0;r<t;r+=8)d(this,r,r+7),d(this,r+1,r+6),d(this,r+2,r+5),d(this,r+3,r+4);return this},s.prototype.toString=function(){var t=this.length;return 0===t?"":0==arguments.length?m(this,0,t):g.apply(this,arguments)},s.prototype.toLocaleString=s.prototype.toString,s.prototype.equals=function(t){if(!s.isBuffer(t))throw TypeError("Argument must be a Buffer");return this===t||0===s.compare(this,t)},s.prototype.inspect=function(){var t="",e=r.INSPECT_MAX_BYTES;return t=this.toString("hex",0,e).replace(/(.{2})/g,"$1 ").trim(),this.length>e&&(t+=" ... "),"<Buffer "+t+">"},f&&(s.prototype[f]=s.prototype.inspect),s.prototype.compare=function(t,r,e,o,i){if(I(t,Uint8Array)&&(t=s.from(t,t.offset,t.byteLength)),!s.isBuffer(t))throw TypeError('The "target" argument must be one of type Buffer or Uint8Array. Received type '+typeof t);if(void 0===r&&(r=0),void 0===e&&(e=t?t.length:0),void 0===o&&(o=0),void 0===i&&(i=this.length),r<0||e>t.length||o<0||i>this.length)throw RangeError("out of range index");if(o>=i&&r>=e)return 0;if(o>=i)return -1;if(r>=e)return 1;if(r>>>=0,e>>>=0,o>>>=0,i>>>=0,this===t)return 0;for(var f=i-o,u=e-r,a=Math.min(f,u),h=this.slice(o,i),c=t.slice(r,e),l=0;l<a;++l)if(h[l]!==c[l]){f=h[l],u=c[l];break}return f<u?-1:u<f?1:0},s.prototype.includes=function(t,r,e){return -1!==this.indexOf(t,r,e)},s.prototype.indexOf=function(t,r,e){return v(this,t,r,e,!0)},s.prototype.lastIndexOf=function(t,r,e){return v(this,t,r,e,!1)},s.prototype.write=function(t,r,e,o){if(void 0===r)o="utf8",e=this.length,r=0;else if(void 0===e&&"string"==typeof r)o=r,e=this.length,r=0;else if(isFinite(r))r>>>=0,isFinite(e)?(e>>>=0,void 0===o&&(o="utf8")):(o=e,e=void 0);else throw Error("Buffer.write(string, encoding, offset[, length]) is no longer supported");var i,f,u,s,a,h,c,l,p,y,g,d,v=this.length-r;if((void 0===e||e>v)&&(e=v),t.length>0&&(e<0||r<0)||r>this.length)throw RangeError("Attempt to write outside buffer bounds");o||(o="utf8");for(var b=!1;;)switch(o){case"hex":return function(t,r,e,o){e=Number(e)||0;var i=t.length-e;o?(o=Number(o))>i&&(o=i):o=i;var f=r.length;o>f/2&&(o=f/2);for(var u=0;u<o;++u){var s=parseInt(r.substr(2*u,2),16);if(s!=s)break;t[e+u]=s}return u}(this,t,r,e);case"utf8":case"utf-8":return a=r,h=e,L(U(t,this.length-a),this,a,h);case"ascii":return c=r,l=e,L(S(t),this,c,l);case"latin1":case"binary":;return i=this,f=t,u=r,s=e,L(S(f),i,u,s);case"base64":return p=r,y=e,L(x(t),this,p,y);case"ucs2":case"ucs-2":case"utf16le":case"utf-16le":return g=r,d=e,L(function(t,r){for(var e,o,i=[],f=0;f<t.length&&!((r-=2)<0);++f)o=(e=t.charCodeAt(f))>>8,i.push(e%256),i.push(o);return i}(t,this.length-g),this,g,d);default:if(b)throw TypeError("Unknown encoding: "+o);o=(""+o).toLowerCase(),b=!0}},s.prototype.toJSON=function(){return{type:"Buffer",data:Array.prototype.slice.call(this._arr||this,0)}},s.prototype.slice=function(t,r){var e=this.length;t=~~t,r=void 0===r?e:~~r,t<0?(t+=e)<0&&(t=0):t>e&&(t=e),r<0?(r+=e)<0&&(r=0):r>e&&(r=e),r<t&&(r=t);var o=this.subarray(t,r);return Object.setPrototypeOf(o,s.prototype),o},s.prototype.readUIntLE=function(t,r,e){t>>>=0,r>>>=0,e||w(t,r,this.length);for(var o=this[t],i=1,f=0;++f<r&&(i*=256);)o+=this[t+f]*i;return o},s.prototype.readUIntBE=function(t,r,e){t>>>=0,r>>>=0,e||w(t,r,this.length);for(var o=this[t+--r],i=1;r>0&&(i*=256);)o+=this[t+--r]*i;return o},s.prototype.readUInt8=function(t,r){return t>>>=0,r||w(t,1,this.length),this[t]},s.prototype.readUInt16LE=function(t,r){return t>>>=0,r||w(t,2,this.length),this[t]|this[t+1]<<8},s.prototype.readUInt16BE=function(t,r){return t>>>=0,r||w(t,2,this.length),this[t]<<8|this[t+1]},s.prototype.readUInt32LE=function(t,r){return t>>>=0,r||w(t,4,this.length),(this[t]|this[t+1]<<8|this[t+2]<<16)+16777216*this[t+3]},s.prototype.readUInt32BE=function(t,r){return t>>>=0,r||w(t,4,this.length),16777216*this[t]+(this[t+1]<<16|this[t+2]<<8|this[t+3])},s.prototype.readIntLE=function(t,r,e){t>>>=0,r>>>=0,e||w(t,r,this.length);for(var o=this[t],i=1,f=0;++f<r&&(i*=256);)o+=this[t+f]*i;return o>=(i*=128)&&(o-=Math.pow(2,8*r)),o},s.prototype.readIntBE=function(t,r,e){t>>>=0,r>>>=0,e||w(t,r,this.length);for(var o=r,i=1,f=this[t+--o];o>0&&(i*=256);)f+=this[t+--o]*i;return f>=(i*=128)&&(f-=Math.pow(2,8*r)),f},s.prototype.readInt8=function(t,r){return(t>>>=0,r||w(t,1,this.length),128&this[t])?-((255-this[t]+1)*1):this[t]},s.prototype.readInt16LE=function(t,r){t>>>=0,r||w(t,2,this.length);var e=this[t]|this[t+1]<<8;return 32768&e?4294901760|e:e},s.prototype.readInt16BE=function(t,r){t>>>=0,r||w(t,2,this.length);var e=this[t+1]|this[t]<<8;return 32768&e?4294901760|e:e},s.prototype.readInt32LE=function(t,r){return t>>>=0,r||w(t,4,this.length),this[t]|this[t+1]<<8|this[t+2]<<16|this[t+3]<<24},s.prototype.readInt32BE=function(t,r){return t>>>=0,r||w(t,4,this.length),this[t]<<24|this[t+1]<<16|this[t+2]<<8|this[t+3]},s.prototype.readFloatLE=function(t,r){return t>>>=0,r||w(t,4,this.length),i.read(this,t,!0,23,4)},s.prototype.readFloatBE=function(t,r){return t>>>=0,r||w(t,4,this.length),i.read(this,t,!1,23,4)},s.prototype.readDoubleLE=function(t,r){return t>>>=0,r||w(t,8,this.length),i.read(this,t,!0,52,8)},s.prototype.readDoubleBE=function(t,r){return t>>>=0,r||w(t,8,this.length),i.read(this,t,!1,52,8)},s.prototype.writeUIntLE=function(t,r,e,o){if(t=+t,r>>>=0,e>>>=0,!o){var i=Math.pow(2,8*e)-1;E(this,t,r,e,i,0)}var f=1,u=0;for(this[r]=255&t;++u<e&&(f*=256);)this[r+u]=t/f&255;return r+e},s.prototype.writeUIntBE=function(t,r,e,o){if(t=+t,r>>>=0,e>>>=0,!o){var i=Math.pow(2,8*e)-1;E(this,t,r,e,i,0)}var f=e-1,u=1;for(this[r+f]=255&t;--f>=0&&(u*=256);)this[r+f]=t/u&255;return r+e},s.prototype.writeUInt8=function(t,r,e){return t=+t,r>>>=0,e||E(this,t,r,1,255,0),this[r]=255&t,r+1},s.prototype.writeUInt16LE=function(t,r,e){return t=+t,r>>>=0,e||E(this,t,r,2,65535,0),this[r]=255&t,this[r+1]=t>>>8,r+2},s.prototype.writeUInt16BE=function(t,r,e){return t=+t,r>>>=0,e||E(this,t,r,2,65535,0),this[r]=t>>>8,this[r+1]=255&t,r+2},s.prototype.writeUInt32LE=function(t,r,e){return t=+t,r>>>=0,e||E(this,t,r,4,4294967295,0),this[r+3]=t>>>24,this[r+2]=t>>>16,this[r+1]=t>>>8,this[r]=255&t,r+4},s.prototype.writeUInt32BE=function(t,r,e){return t=+t,r>>>=0,e||E(this,t,r,4,4294967295,0),this[r]=t>>>24,this[r+1]=t>>>16,this[r+2]=t>>>8,this[r+3]=255&t,r+4},s.prototype.writeIntLE=function(t,r,e,o){if(t=+t,r>>>=0,!o){var i=Math.pow(2,8*e-1);E(this,t,r,e,i-1,-i)}var f=0,u=1,s=0;for(this[r]=255&t;++f<e&&(u*=256);)t<0&&0===s&&0!==this[r+f-1]&&(s=1),this[r+f]=(t/u>>0)-s&255;return r+e},s.prototype.writeIntBE=function(t,r,e,o){if(t=+t,r>>>=0,!o){var i=Math.pow(2,8*e-1);E(this,t,r,e,i-1,-i)}var f=e-1,u=1,s=0;for(this[r+f]=255&t;--f>=0&&(u*=256);)t<0&&0===s&&0!==this[r+f+1]&&(s=1),this[r+f]=(t/u>>0)-s&255;return r+e},s.prototype.writeInt8=function(t,r,e){return t=+t,r>>>=0,e||E(this,t,r,1,127,-128),t<0&&(t=255+t+1),this[r]=255&t,r+1},s.prototype.writeInt16LE=function(t,r,e){return t=+t,r>>>=0,e||E(this,t,r,2,32767,-32768),this[r]=255&t,this[r+1]=t>>>8,r+2},s.prototype.writeInt16BE=function(t,r,e){return t=+t,r>>>=0,e||E(this,t,r,2,32767,-32768),this[r]=t>>>8,this[r+1]=255&t,r+2},s.prototype.writeInt32LE=function(t,r,e){return t=+t,r>>>=0,e||E(this,t,r,4,2147483647,-2147483648),this[r]=255&t,this[r+1]=t>>>8,this[r+2]=t>>>16,this[r+3]=t>>>24,r+4},s.prototype.writeInt32BE=function(t,r,e){return t=+t,r>>>=0,e||E(this,t,r,4,2147483647,-2147483648),t<0&&(t=4294967295+t+1),this[r]=t>>>24,this[r+1]=t>>>16,this[r+2]=t>>>8,this[r+3]=255&t,r+4},s.prototype.writeFloatLE=function(t,r,e){return B(this,t,r,!0,e)},s.prototype.writeFloatBE=function(t,r,e){return B(this,t,r,!1,e)},s.prototype.writeDoubleLE=function(t,r,e){return T(this,t,r,!0,e)},s.prototype.writeDoubleBE=function(t,r,e){return T(this,t,r,!1,e)},s.prototype.copy=function(t,r,e,o){if(!s.isBuffer(t))throw TypeError("argument should be a Buffer");if(e||(e=0),o||0===o||(o=this.length),r>=t.length&&(r=t.length),r||(r=0),o>0&&o<e&&(o=e),o===e||0===t.length||0===this.length)return 0;if(r<0)throw RangeError("targetStart out of bounds");if(e<0||e>=this.length)throw RangeError("Index out of range");if(o<0)throw RangeError("sourceEnd out of bounds");o>this.length&&(o=this.length),t.length-r<o-e&&(o=t.length-r+e);var i=o-e;if(this===t&&"function"==typeof Uint8Array.prototype.copyWithin)this.copyWithin(r,e,o);else if(this===t&&e<r&&r<o)for(var f=i-1;f>=0;--f)t[f+r]=this[f+e];else Uint8Array.prototype.set.call(t,this.subarray(e,o),r);return i},s.prototype.fill=function(t,r,e,o){if("string"==typeof t){if("string"==typeof r?(o=r,r=0,e=this.length):"string"==typeof e&&(o=e,e=this.length),void 0!==o&&"string"!=typeof o)throw TypeError("encoding must be a string");if("string"==typeof o&&!s.isEncoding(o))throw TypeError("Unknown encoding: "+o);if(1===t.length){var i,f=t.charCodeAt(0);("utf8"===o&&f<128||"latin1"===o)&&(t=f)}}else"number"==typeof t?t&=255:"boolean"==typeof t&&(t=Number(t));if(r<0||this.length<r||this.length<e)throw RangeError("Out of range index");if(e<=r)return this;if(r>>>=0,e=void 0===e?this.length:e>>>0,t||(t=0),"number"==typeof t)for(i=r;i<e;++i)this[i]=t;else{var u=s.isBuffer(t)?t:s.from(t,o),a=u.length;if(0===a)throw TypeError('The value "'+t+'" is invalid for argument "value"');for(i=0;i<e-r;++i)this[i+r]=u[i%a]}return this};var O=/[^+/0-9A-Za-z-_]/g;function U(t,r){r=r||1/0;for(var e,o=t.length,i=null,f=[],u=0;u<o;++u){if((e=t.charCodeAt(u))>55295&&e<57344){if(!i){if(e>56319||u+1===o){(r-=3)>-1&&f.push(239,191,189);continue}i=e;continue}if(e<56320){(r-=3)>-1&&f.push(239,191,189),i=e;continue}e=(i-55296<<10|e-56320)+65536}else i&&(r-=3)>-1&&f.push(239,191,189);if(i=null,e<128){if((r-=1)<0)break;f.push(e)}else if(e<2048){if((r-=2)<0)break;f.push(e>>6|192,63&e|128)}else if(e<65536){if((r-=3)<0)break;f.push(e>>12|224,e>>6&63|128,63&e|128)}else if(e<1114112){if((r-=4)<0)break;f.push(e>>18|240,e>>12&63|128,e>>6&63|128,63&e|128)}else throw Error("Invalid code point")}return f}function S(t){for(var r=[],e=0;e<t.length;++e)r.push(255&t.charCodeAt(e));return r}function x(t){return o.toByteArray(function(t){if((t=(t=t.split("=")[0]).trim().replace(O,"")).length<2)return"";for(;t.length%4!=0;)t+="=";return t}(t))}function L(t,r,e,o){for(var i=0;i<o&&!(i+e>=r.length)&&!(i>=t.length);++i)r[i+e]=t[i];return i}function I(t,r){return t instanceof r||null!=t&&null!=t.constructor&&null!=t.constructor.name&&t.constructor.name===r.name}var R=function(){for(var t="0123456789abcdef",r=Array(256),e=0;e<16;++e)for(var o=16*e,i=0;i<16;++i)r[o+i]=t[e]+t[i];return r}()},783:function(t,r){/*! ieee754. BSD-3-Clause License. Feross Aboukhadijeh <https://feross.org/opensource> */r.read=function(t,r,e,o,i){var f,u,s=8*i-o-1,a=(1<<s)-1,h=a>>1,c=-7,l=e?i-1:0,p=e?-1:1,y=t[r+l];for(l+=p,f=y&(1<<-c)-1,y>>=-c,c+=s;c>0;f=256*f+t[r+l],l+=p,c-=8);for(u=f&(1<<-c)-1,f>>=-c,c+=o;c>0;u=256*u+t[r+l],l+=p,c-=8);if(0===f)f=1-h;else{if(f===a)return u?NaN:(y?-1:1)*(1/0);u+=Math.pow(2,o),f-=h}return(y?-1:1)*u*Math.pow(2,f-o)},r.write=function(t,r,e,o,i,f){var u,s,a,h=8*f-i-1,c=(1<<h)-1,l=c>>1,p=23===i?5960464477539062e-23:0,y=o?0:f-1,g=o?1:-1,d=r<0||0===r&&1/r<0?1:0;for(isNaN(r=Math.abs(r))||r===1/0?(s=isNaN(r)?1:0,u=c):(u=Math.floor(Math.log(r)/Math.LN2),r*(a=Math.pow(2,-u))<1&&(u--,a*=2),u+l>=1?r+=p/a:r+=p*Math.pow(2,1-l),r*a>=2&&(u++,a/=2),u+l>=c?(s=0,u=c):u+l>=1?(s=(r*a-1)*Math.pow(2,i),u+=l):(s=r*Math.pow(2,l-1)*Math.pow(2,i),u=0));i>=8;t[e+y]=255&s,y+=g,s/=256,i-=8);for(u=u<<i|s,h+=i;h>0;t[e+y]=255&u,y+=g,u/=256,h-=8);t[e+y-g]|=128*d}}},e={};function o(t){var i=e[t];if(void 0!==i)return i.exports;var f=e[t]={exports:{}},u=!0;try{r[t](f,f.exports,o),u=!1}finally{u&&delete e[t]}return f.exports}o.ab="//";var i=o(72);t.exports=i}()},7663:function(t){!function(){var r={229:function(t){var r,e,o,i=t.exports={};function f(){throw Error("setTimeout has not been defined")}function u(){throw Error("clearTimeout has not been defined")}function s(t){if(r===setTimeout)return setTimeout(t,0);if((r===f||!r)&&setTimeout)return r=setTimeout,setTimeout(t,0);try{return r(t,0)}catch(o){try{return r.call(null,t,0)}catch(e){return r.call(this,t,0)}}}!function(){try{r="function"==typeof setTimeout?setTimeout:f}catch(t){r=f}try{e="function"==typeof clearTimeout?clearTimeout:u}catch(o){e=u}}();var a=[],h=!1,c=-1;function l(){h&&o&&(h=!1,o.length?a=o.concat(a):c=-1,a.length&&p())}function p(){if(!h){var t=s(l);h=!0;for(var r=a.length;r;){for(o=a,a=[];++c<r;)o&&o[c].run();c=-1,r=a.length}o=null,h=!1,function(t){if(e===clearTimeout)return clearTimeout(t);if((e===u||!e)&&clearTimeout)return e=clearTimeout,clearTimeout(t);try{e(t)}catch(o){try{return e.call(null,t)}catch(r){return e.call(this,t)}}}(t)}}function y(t,r){this.fun=t,this.array=r}function g(){}i.nextTick=function(t){var r=Array(arguments.length-1);if(arguments.length>1)for(var e=1;e<arguments.length;e++)r[e-1]=arguments[e];a.push(new y(t,r)),1!==a.length||h||s(p)},y.prototype.run=function(){this.fun.apply(null,this.array)},i.title="browser",i.browser=!0,i.env={},i.argv=[],i.version="",i.versions={},i.on=g,i.addListener=g,i.once=g,i.off=g,i.removeListener=g,i.removeAllListeners=g,i.emit=g,i.prependListener=g,i.prependOnceListener=g,i.listeners=function(t){return[]},i.binding=function(t){throw Error("process.binding is not supported")},i.cwd=function(){return"/"},i.chdir=function(t){throw Error("process.chdir is not supported")},i.umask=function(){return 0}}},e={};function o(t){var i=e[t];if(void 0!==i)return i.exports;var f=e[t]={exports:{}},u=!0;try{r[t](f,f.exports,o),u=!1}finally{u&&delete e[t]}return f.exports}o.ab="//";var i=o(229);t.exports=i}()},9849:function(t,r,e){"use strict";e.r(r),e.d(r,{default:function(){return R}});var o,i=e(7294),f=e(2249),u="u">typeof globalThis?globalThis:"u">typeof window?window:"u">typeof global?global:"u">typeof self?self:{},s={},a={get exports(){return s},set exports(n){s=n}},h={};a.exports=function(){if(o)return h;o=1;var t=Symbol.for("react.element"),r=Symbol.for("react.fragment"),e=Object.prototype.hasOwnProperty,f=i.__SECRET_INTERNALS_DO_NOT_USE_OR_YOU_WILL_BE_FIRED.ReactCurrentOwner,u={key:!0,ref:!0,__self:!0,__source:!0};function s(r,o,i){var s,a={},h=null,c=null;for(s in void 0!==i&&(h=""+i),void 0!==o.key&&(h=""+o.key),void 0!==o.ref&&(c=o.ref),o)e.call(o,s)&&!u.hasOwnProperty(s)&&(a[s]=o[s]);if(r&&r.defaultProps)for(s in o=r.defaultProps)void 0===a[s]&&(a[s]=o[s]);return{$$typeof:t,type:r,key:h,ref:c,props:a,_owner:f.current}}return h.Fragment=r,h.jsx=s,h.jsxs=s,h}();let c=s.jsx;var l=0/0,p=/^\s+|\s+$/g,y=/^[-+]0x[0-9a-f]+$/i,g=/^0b[01]+$/i,d=/^0o[0-7]+$/i,v=parseInt,b="object"==typeof u&&u&&u.Object===Object&&u,m="object"==typeof self&&self&&self.Object===Object&&self,w=b||m||Function("return this")(),E=Object.prototype.toString,A=Math.max,B=Math.min,T=function(){return w.Date.now()};function O(t){var r=typeof t;return!!t&&("object"==r||"function"==r)}function U(t){if("number"==typeof t)return t;if("symbol"==typeof(r=t)||(e=r)&&"object"==typeof e&&"[object Symbol]"==E.call(r))return l;if(O(t)){var r,e,o="function"==typeof t.valueOf?t.valueOf():t;t=O(o)?o+"":o}if("string"!=typeof t)return 0===t?t:+t;t=t.replace(p,"");var i=g.test(t);return i||d.test(t)?v(t.slice(2),i?2:8):y.test(t)?l:+t}var S=function(t,r,e){var o,i,f,u,s,a,h=0,c=!1,l=!1,p=!0;if("function"!=typeof t)throw TypeError("Expected a function");function y(r){var e=o,f=i;return o=i=void 0,h=r,u=t.apply(f,e)}function g(t){var e=t-a,o=t-h;return void 0===a||e>=r||e<0||l&&o>=f}function d(){var t,e,o,i=T();if(g(i))return v(i);s=setTimeout(d,(t=i-a,e=i-h,o=r-t,l?B(o,f-e):o))}function v(t){return s=void 0,p&&o?y(t):(o=i=void 0,u)}function b(){var t,e=T(),f=g(e);if(o=arguments,i=this,a=e,f){if(void 0===s)return h=t=a,s=setTimeout(d,r),c?y(t):u;if(l)return s=setTimeout(d,r),y(a)}return void 0===s&&(s=setTimeout(d,r)),u}return r=U(r)||0,O(e)&&(c=!!e.leading,f=(l="maxWait"in e)?A(U(e.maxWait)||0,r):f,p="trailing"in e?!!e.trailing:p),b.cancel=function(){void 0!==s&&clearTimeout(s),h=0,o=a=i=s=void 0},b.flush=function(){return void 0===s?u:v(T())},b};let x=[],L={width:"100%",height:"100%"},I=(0,i.forwardRef)(function({className:t,children:r,debounceTime:e=300,ignoreDimensions:o=x,parentSizeStyles:f,enableDebounceLeadingCall:u=!0,resizeObserverPolyfill:s,...a},h){var l;let p=(0,i.useRef)(null),y=(0,i.useRef)(0),[g,d]=(0,i.useState)({width:0,height:0,top:0,left:0}),v=(0,i.useMemo)(()=>{let t=Array.isArray(o)?o:[o];return S(r=>{d(e=>Object.keys(e).filter(t=>e[t]!==r[t]).every(r=>t.includes(r))?e:r)},e,{leading:u})},[e,u,o]);return(0,i.useEffect)(()=>{let t=s||window.ResizeObserver,r=new t(t=>{t.forEach(t=>{let{left:r,top:e,width:o,height:i}=(null==t?void 0:t.contentRect)??{};y.current=window.requestAnimationFrame(()=>{v({width:o,height:i,top:e,left:r})})})});return p.current&&r.observe(p.current),()=>{window.cancelAnimationFrame(y.current),r.disconnect(),v.cancel()}},[v,s]),c("div",{style:{...L,...f},ref:(l=[h,p],t=>{l.forEach(r=>{"function"==typeof r?r(t):null!=r&&(r.current=t)})}),className:t,...a,children:r({...g,ref:p.current,resize:v})})}),R=(0,i.forwardRef)(({scene:t,style:r,onMouseDown:e,onMouseUp:o,onMouseHover:u,onKeyDown:s,onKeyUp:a,onStart:h,onLookAt:l,onFollow:p,onWheel:y,onLoad:g,renderOnDemand:d=!0,...v},b)=>{let m=(0,i.useRef)(null),[w,E]=(0,i.useState)(!0);return(0,i.useEffect)(()=>{let r;E(!0);let i=[{name:"mouseDown",cb:e},{name:"mouseUp",cb:o},{name:"mouseHover",cb:u},{name:"keyDown",cb:s},{name:"keyUp",cb:a},{name:"start",cb:h},{name:"lookAt",cb:l},{name:"follow",cb:p},{name:"scroll",cb:y}];return m.current&&(r=new f.M(m.current,{renderOnDemand:d}),async function(){for(let e of(await r.load(t),i))e.cb&&r.addEventListener(e.name,e.cb);E(!1),null==g||g(r)}()),()=>{for(let t of i)t.cb&&r.removeEventListener(t.name,t.cb);r.dispose()}},[t]),c(I,{ref:b,parentSizeStyles:r,debounceTime:50,...v,children:()=>c("canvas",{ref:m,style:{display:w?"none":"block"}})})})}}]);