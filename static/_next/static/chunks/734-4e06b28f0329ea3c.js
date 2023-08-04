"use strict";(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[734],{3250:function(e,t,n){/**
 * @license React
 * use-sync-external-store-shim.production.min.js
 *
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */var r=n(7294),i="function"==typeof Object.is?Object.is:function(e,t){return e===t&&(0!==e||1/e==1/t)||e!=e&&t!=t},a=r.useState,u=r.useEffect,o=r.useLayoutEffect,l=r.useDebugValue;function s(e){var t=e.getSnapshot;e=e.value;try{var n=t();return!i(e,n)}catch(r){return!0}}var c="undefined"==typeof window||void 0===window.document||void 0===window.document.createElement?function(e,t){return t()}:function(e,t){var n=t(),r=a({inst:{value:n,getSnapshot:t}}),i=r[0].inst,c=r[1];return o(function(){i.value=n,i.getSnapshot=t,s(i)&&c({inst:i})},[e,n,t]),u(function(){return s(i)&&c({inst:i}),e(function(){s(i)&&c({inst:i})})},[e]),l(n),n};t.useSyncExternalStore=void 0!==r.useSyncExternalStore?r.useSyncExternalStore:c},1688:function(e,t,n){e.exports=n(3250)},9734:function(e,t,n){n.d(t,{ZP:function(){return ei}});var r=n(7294),i=n(1688);let a=new WeakMap,u={},o=()=>{},l=o(),s=Object,c=e=>e===l,d=e=>"function"==typeof e,f=(e,t)=>({...e,...t}),g="undefined",E=typeof window!=g,w=typeof document!=g,v=()=>E&&typeof window.requestAnimationFrame!=g,p=(e,t)=>{let n=a.get(e);return[()=>e.get(t)||u,r=>{let i=e.get(t);n[5](t,f(i,r),i||u)},n[6]]},h=new WeakMap,y=0,_=e=>{let t,n;let r=typeof e,i=e&&e.constructor,a=i==Date;if(s(e)!==e||a||i==RegExp)t=a?e.toJSON():"symbol"==r?e.toString():"string"==r?JSON.stringify(e):""+e;else{if(t=h.get(e))return t;if(t=++y+"~",h.set(e,t),i==Array){for(n=0,t="@";n<e.length;n++)t+=_(e[n])+",";h.set(e,t)}if(i==s){t="#";let u=s.keys(e).sort();for(;!c(n=u.pop());)c(e[n])||(t+=n+":"+_(e[n])+",");h.set(e,t)}}return t},S=!0,[m,b]=E&&window.addEventListener?[window.addEventListener.bind(window),window.removeEventListener.bind(window)]:[o,o],T=()=>{let e=w&&document.visibilityState;return c(e)||"hidden"!==e},O=e=>(w&&document.addEventListener("visibilitychange",e),m("focus",e),()=>{w&&document.removeEventListener("visibilitychange",e),b("focus",e)}),R=e=>{let t=()=>{S=!0,e()},n=()=>{S=!1};return m("online",t),m("offline",n),()=>{b("online",t),b("offline",n)}},k={initFocus:O,initReconnect:R},C=!r.useId,V=!E||"Deno"in window,L=e=>v()?window.requestAnimationFrame(e):setTimeout(e,1),N=V?r.useEffect:r.useLayoutEffect,x="undefined"!=typeof navigator&&navigator.connection,D=!V&&x&&(["slow-2g","2g"].includes(x.effectiveType)||x.saveData),I=e=>{if(d(e))try{e=e()}catch(t){e=""}let n=e;return[e="string"==typeof e?e:(Array.isArray(e)?e.length:e)?_(e):"",n]},M=0,P=()=>++M;var A={__proto__:null,FOCUS_EVENT:0,RECONNECT_EVENT:1,MUTATE_EVENT:2};async function F(...e){let[t,n,r,i]=e,u=f({populateCache:!0,throwOnError:!0},"boolean"==typeof i?{revalidate:i}:i||{}),o=u.populateCache,s=u.rollbackOnError,g=u.optimisticData,E=!1!==u.revalidate,w=e=>"function"==typeof s?s(e):!1!==s,v=u.throwOnError;if(d(n)){let h=[],y=t.keys();for(let _=y.next();!_.done;_=y.next()){let S=_.value;!S.startsWith("$inf$")&&n(t.get(S)._k)&&h.push(S)}return Promise.all(h.map(m))}return m(n);async function m(n){let i;let[u]=I(n);if(!u)return;let[s,f]=p(t,u),[h,y,_]=a.get(t),S=h[u],m=()=>E&&(delete _[u],S&&S[0])?S[0](2).then(()=>s().data):s().data;if(e.length<3)return m();let b=r,T=P();y[u]=[T,0];let O=!c(g),R=s(),k=R.data,C=R._c,V=c(C)?k:C;if(O&&f({data:g=d(g)?g(V):g,_c:V}),d(b))try{b=b(V)}catch(L){i=L}if(b&&d(b.then)){if(b=await b.catch(e=>{i=e}),T!==y[u][0]){if(i)throw i;return b}i&&O&&w(i)&&(o=!0,b=V,f({data:b,_c:l}))}o&&!i&&(d(o)&&(b=o(b,V)),f({data:b,_c:l})),y[u][1]=P();let N=await m();if(f({_c:l}),i){if(v)throw i;return}return o?N:b}}let U=(e,t)=>{for(let n in e)e[n][0]&&e[n][0](t)},W=(e,t)=>{if(!a.has(e)){let n=f(k,t),r={},i=F.bind(l,e),u=o,s={},c=(e,t)=>{let n=s[e]||[];return s[e]=n,n.push(t),()=>n.splice(n.indexOf(t),1)},d=(t,n,r)=>{e.set(t,n);let i=s[t];if(i)for(let a=i.length;a--;)i[a](r,n)},g=()=>{if(!a.has(e)&&(a.set(e,[r,{},{},{},i,d,c]),!V)){let t=n.initFocus(setTimeout.bind(l,U.bind(l,r,0))),o=n.initReconnect(setTimeout.bind(l,U.bind(l,r,1)));u=()=>{t&&t(),o&&o(),a.delete(e)}}};return g(),[e,i,g,u]}return[e,a.get(e)[4]]},j=(e,t,n,r,i)=>{let a=n.errorRetryCount,u=i.retryCount,o=~~((Math.random()+.5)*(1<<(u<8?u:8)))*n.errorRetryInterval;(c(a)||!(u>a))&&setTimeout(r,o,i)},q=(e,t)=>_(e)==_(t),[J,$]=W(new Map),Z=f({onLoadingSlow:o,onSuccess:o,onError:o,onErrorRetry:j,onDiscarded:o,revalidateOnFocus:!0,revalidateOnReconnect:!0,revalidateIfStale:!0,shouldRetryOnError:!0,errorRetryInterval:D?1e4:5e3,focusThrottleInterval:5e3,dedupingInterval:2e3,loadingTimeout:D?5e3:3e3,compare:q,isPaused:()=>!1,cache:J,mutate:$,fallback:{}},{isOnline:()=>S,isVisible:T}),z=(e,t)=>{let n=f(e,t);if(t){let{use:r,fallback:i}=e,{use:a,fallback:u}=t;r&&a&&(n.use=r.concat(a)),i&&u&&(n.fallback=f(i,u))}return n},B=(0,r.createContext)({}),G=e=>{let{value:t}=e,n=(0,r.useContext)(B),i=d(t),a=(0,r.useMemo)(()=>i?t(n):t,[i,n,t]),u=(0,r.useMemo)(()=>i?a:z(n,a),[i,n,a]),o=a&&a.provider,[s]=(0,r.useState)(()=>o?W(o(u.cache||J),a):l);return s&&(u.cache=s[0],u.mutate=s[1]),N(()=>{if(s)return s[2]&&s[2](),s[3]},[]),(0,r.createElement)(B.Provider,f(e,{value:u}))},H=E&&window.__SWR_DEVTOOLS_USE__,K=H?window.__SWR_DEVTOOLS_USE__:[],Q=e=>d(e[1])?[e[0],e[1],e[2]||{}]:[e[0],null,(null===e[1]?e[2]:e[1])||{}],X=()=>f(Z,(0,r.useContext)(B)),Y=e=>(t,n,r)=>{let i=n&&((...e)=>{let r=I(t)[0],[,,,i]=a.get(J),u=i[r];return u?(delete i[r],u):n(...e)});return e(t,i,r)},ee=K.concat(Y),et=(e,t,n)=>{let r=t[e]||(t[e]=[]);return r.push(n),()=>{let e=r.indexOf(n);e>=0&&(r[e]=r[r.length-1],r.pop())}};H&&(window.__SWR_DEVTOOLS_REACT__=r);let en={dedupe:!0},er=(e,t,n)=>{let{cache:u,compare:o,suspense:s,fallbackData:g,revalidateOnMount:E,refreshInterval:w,refreshWhenHidden:v,refreshWhenOffline:h,keepPreviousData:y}=n,[_,S,m]=a.get(u),[b,T]=I(e),O=(0,r.useRef)(!1),R=(0,r.useRef)(!1),k=(0,r.useRef)(b),x=(0,r.useRef)(t),D=(0,r.useRef)(n),M=()=>D.current,U=()=>M().isVisible()&&M().isOnline(),[W,j,q]=p(u,b),J=(0,r.useRef)({}).current,$=c(g)?n.fallback[b]:g,Z=(e,t)=>{let n=!0;for(let r in J){let i=r;o(t[i],e[i])||"data"===i&&c(e[i])&&o(t[i],Y)||(n=!1)}return n},z=(0,r.useMemo)(()=>{let e=!!b&&!!t&&(c(E)?!M().isPaused()&&!s:E),n=()=>{let t=W(),n=f(t);return(delete n._k,e)?{isValidating:!0,isLoading:!0,...n}:n},r=n();return()=>{let e=n();return Z(e,r)?r:r=e}},[u,b]),B=(0,i.useSyncExternalStore)((0,r.useCallback)(e=>q(b,(t,n)=>{Z(t,n)||e()}),[u,b]),z,z),G=!O.current,H=B.data,K=c(H)?$:H,Q=B.error,X=(0,r.useRef)(K),Y=y?c(H)?X.current:H:K,ee=G&&!c(E)?E:!M().isPaused()&&(s?!c(K)&&n.revalidateIfStale:c(K)||n.revalidateIfStale),er=!!(b&&t&&G&&ee),ei=c(B.isValidating)?er:B.isValidating,ea=c(B.isLoading)?er:B.isLoading,eu=(0,r.useCallback)(async e=>{let t,r;let i=x.current;if(!b||!i||R.current||M().isPaused())return!1;let a=!0,u=e||{},s=!m[b]||!u.dedupe,f=()=>C?!R.current&&b===k.current&&O.current:b===k.current,g={isValidating:!1,isLoading:!1},E=()=>{j(g)},w=()=>{let e=m[b];e&&e[1]===r&&delete m[b]},v={isValidating:!0};c(W().data)&&(v.isLoading=!0);try{if(s&&(j(v),n.loadingTimeout&&c(W().data)&&setTimeout(()=>{a&&f()&&M().onLoadingSlow(b,n)},n.loadingTimeout),m[b]=[i(T),P()]),[t,r]=m[b],t=await t,s&&setTimeout(w,n.dedupingInterval),!m[b]||m[b][1]!==r)return s&&f()&&M().onDiscarded(b),!1;g.error=l;let p=S[b];if(!c(p)&&(r<=p[0]||r<=p[1]||0===p[1]))return E(),s&&f()&&M().onDiscarded(b),!1;let h=W().data;g.data=o(h,t)?h:t,s&&f()&&M().onSuccess(t,b,n)}catch(V){w();let y=M(),{shouldRetryOnError:_}=y;!y.isPaused()&&(g.error=V,s&&f()&&(y.onError(V,b,y),(!0===_||d(_)&&_(V))&&U()&&y.onErrorRetry(V,b,y,eu,{retryCount:(u.retryCount||0)+1,dedupe:!0})))}return a=!1,E(),!0},[b,u]),eo=(0,r.useCallback)((...e)=>F(u,k.current,...e),[]);if(N(()=>{x.current=t,D.current=n,c(H)||(X.current=H)}),N(()=>{if(!b)return;let e=eu.bind(l,en),t=0,n=n=>{if(n==A.FOCUS_EVENT){let r=Date.now();M().revalidateOnFocus&&r>t&&U()&&(t=r+M().focusThrottleInterval,e())}else if(n==A.RECONNECT_EVENT)M().revalidateOnReconnect&&U()&&e();else if(n==A.MUTATE_EVENT)return eu()},r=et(b,_,n);return R.current=!1,k.current=b,O.current=!0,j({_k:T}),ee&&(c(K)||V?e():L(e)),()=>{R.current=!0,r()}},[b]),N(()=>{let e;function t(){let t=d(w)?w(K):w;t&&-1!==e&&(e=setTimeout(n,t))}function n(){!W().error&&(v||M().isVisible())&&(h||M().isOnline())?eu(en).then(t):t()}return t(),()=>{e&&(clearTimeout(e),e=-1)}},[w,v,h,b]),(0,r.useDebugValue)(Y),s&&c(K)&&b){if(!C&&V)throw Error("Fallback data is required when using suspense in SSR.");throw x.current=t,D.current=n,R.current=!1,c(Q)?eu(en):Q}return{mutate:eo,get data(){return J.data=!0,Y},get error(){return J.error=!0,Q},get isValidating(){return J.isValidating=!0,ei},get isLoading(){return J.isLoading=!0,ea}}};s.defineProperty(G,"defaultValue",{value:Z});var ei=function(...e){let t=X(),[n,r,i]=Q(e),a=z(t,i),u=er,{use:o}=a,l=(o||[]).concat(ee);for(let s=l.length;s--;)u=l[s](u);return u(n,r||a.fetcher||null,a)}}}]);