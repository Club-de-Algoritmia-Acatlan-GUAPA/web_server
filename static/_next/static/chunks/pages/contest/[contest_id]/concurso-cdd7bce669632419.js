(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[573],{2042:function(e,t,r){(window.__NEXT_P=window.__NEXT_P||[]).push(["/contest/[contest_id]/concurso",function(){return r(8974)}])},1571:function(e,t,r){"use strict";r.d(t,{dI:function(){return m},Y:function(){return f}});var n,l,i=r(5893),c=r(9734),a=r(8241),s=r(7294),o=r(1163),u=r(4869);function d(e){let{selectedTab:t}=e,r=(0,o.useRouter)(),n=r.asPath.split("/").slice(-1)[0],c=f();return(0,i.jsx)(i.Fragment,{children:(0,i.jsx)("div",{className:"flex gap-7 items-center",children:Object.keys(l).map(e=>{let t=l[e];return(0,i.jsx)(u.Z,{className:t===n?"selected":"",type:"h3",weight:"bold",clickable:!0,color:t===n?"blue":"black",href:"/contest/".concat(null==c?void 0:c.id,"/").concat(t),children:e},t)})})})}(n=l||(l={})).Problemas="problems",n.Scoreboard="scoreboard",n.Concurso="concurso",n.Submits="submits",n.Aclaraciones="acla";let h=(0,s.createContext)({id:"",name:"",startTime:void 0,endTime:void 0,bases:[]});function f(){return(0,s.useContext)(h)}function x(e){let{children:t}=e,r=(0,o.useRouter)(),{contest_id:n}=r.query,{data:l,error:s}=(0,c.ZP)("/api/contest/".concat(n),a._i);return s?(0,i.jsx)("div",{children:"Failed to load"}):l?(0,i.jsx)(i.Fragment,{children:(0,i.jsx)(h.Provider,{value:l,children:t})}):(0,i.jsx)("div",{children:"Loading..."})}function m(e){return(0,i.jsx)(i.Fragment,{children:(0,i.jsx)("div",{className:"flex flex-col gap-6 layout",children:(0,i.jsxs)(x,{children:[(0,i.jsx)(d,{}),e]})})})}},8974:function(e,t,r){"use strict";r.r(t),r.d(t,{default:function(){return h}});var n=r(5893),l=r(1163),i=r(4869),c=r(7294);function a(e){let{startTime:t,endTime:r}=e;(0,c.useRef)(null);let[l,a]=(0,c.useState)("0"),[s,o]=(0,c.useState)(!1);(0,c.useEffect)(()=>{u();let e=setInterval(u,1e3);return()=>clearInterval(e)},[t,r]);let u=()=>{let e=new Date;if(r){let n=new Date(r).getTime()-e.getTime();if(t){let l=(1-n/(new Date(r).getTime()-new Date(t).getTime()))*100;l<=100?a("".concat(l)):s||o(!0)}}};return(0,n.jsx)(n.Fragment,{children:(0,n.jsx)("div",{className:"flex flex-col w-full",children:s?(0,n.jsx)(n.Fragment,{children:(0,n.jsxs)("div",{className:"flex justify-center gap-2",children:[(0,n.jsx)(i.Z,{type:"h2",weight:"regular",children:" El concurso ha terminado"}),(0,n.jsx)(i.Z,{type:"h2",color:"blue",weight:"regular",clickable:!0,children:" ver scoreboard TODO"})]})}):(0,n.jsxs)(n.Fragment,{children:[(0,n.jsx)("div",{className:"h-5 w-full relative bg-[#D9D9D9] rounded-full"}),(0,n.jsx)("div",{style:{width:l+"%"},className:"relative z-3 h-5 top-[-1.25rem] bg-[#4AAE4B] rounded-full"})]})})})}function s(e){let{name:t,startTime:r,endTime:l}=e;function c(e){return e?e.toString():""}return(0,n.jsx)(n.Fragment,{children:(0,n.jsxs)("div",{className:"flex flex-col w-full items-center",children:[(0,n.jsx)(i.Z,{type:"h1",weight:"regular",children:t}),(0,n.jsxs)("div",{className:"flex justify-between w-full",children:[(0,n.jsxs)("div",{className:"flex gap-4",children:[(0,n.jsx)(i.Z,{type:"h2",weight:"light",children:"Inicio : "}),(0,n.jsx)(i.Z,{type:"h2",children:c(r)})]}),(0,n.jsxs)("div",{className:"flex gap-4",children:[(0,n.jsx)(i.Z,{type:"h2",weight:"light",children:"Final : "}),(0,n.jsx)(i.Z,{type:"h2",children:c(l)})]})]}),(0,n.jsx)(a,{startTime:r,endTime:l}),(0,n.jsxs)("div",{className:"flex justify-between w-full",children:[(0,n.jsxs)("div",{className:"flex gap-4",children:[(0,n.jsx)(i.Z,{type:"h2",weight:"light",children:"Tiempo transcurrido : "}),(0,n.jsxs)(i.Z,{type:"h2",children:[" ","TODO"]})]}),(0,n.jsxs)("div",{className:"flex gap-4",children:[(0,n.jsx)(i.Z,{type:"h2",weight:"light",children:"Tiempo restante : "}),(0,n.jsx)(i.Z,{type:"h2",children:"TODO"})]})]})]})})}function o(e){let{title:t="Prueba",content:r=""}=e;return(0,n.jsx)(n.Fragment,{children:(0,n.jsxs)("div",{className:" bg-[var(--primary-bg-color)] rounded-[var(--border-radius)] ",children:[(0,n.jsx)("div",{className:" text-center bg-[var(--secondary-bg-color)] rounded-t-[var(--border-radius)] ",children:(0,n.jsx)(i.Z,{type:"h2",weight:"black",children:t})}),(0,n.jsx)("div",{className:" p-[var(--primary-padding)] ",children:r})]},t)})}var u=r(1571);let d=()=>{var e,t;let r=(0,l.useRouter)(),{contest_id:i}=r.query,c=(0,u.Y)();return"unauthenticated"===status?(0,n.jsx)("div",{children:"logeate"}):"loading"===status?(0,n.jsx)("div",{children:"Loading"}):(0,n.jsxs)(n.Fragment,{children:[(0,n.jsx)("div",{className:"flex flex-col bg-[#E7EBEF] p-[20px] rounded-[10px] gap-[5px]",children:(0,n.jsx)(s,{name:null!==(t=null==c?void 0:c.name)&&void 0!==t?t:"",startTime:null==c?void 0:c.startTime,endTime:null==c?void 0:c.endTime})}),null==c?void 0:null===(e=c.bases)||void 0===e?void 0:e.map(e=>{let{title:t,content:r}=e;return(0,n.jsx)(o,{title:t,content:r},t)})]})};d.getLayout=u.dI;var h=d},8241:function(e,t,r){"use strict";async function n(e){try{let t=await fetch("".concat("http://143.198.77.99:8000","/submission-get?problem_id=").concat(e,"&from=0&to=0"),{headers:{credentials:"include",withCredentials:"true"}});if(console.log(t),t.redirected)return window.location.replace(t.url),{ok:null,err:"redirected"};if(200!==t.status){let r=await t.text();return console.log(r),{ok:r,err:null}}let n=await t.json();return console.log("infoooooooooooo",n),{ok:n,err:null}}catch(l){return{ok:null,err:l}}}async function l(e){try{let t=await fetch("".concat("http://143.198.77.99:8000","/problem-get?id=").concat(e),{cache:"force-cache"});if(console.log(t),t.redirected)return{ok:null,err:"redirected"};if(200!==t.status){let r=await t.text();console.log(r)}let n=await t.json();return console.log(n),{ok:n,err:null}}catch(l){return{ok:null,err:l}}}r.d(t,{C3:function(){return l},Md:function(){return a},Tq:function(){return n},XH:function(){return i},_i:function(){return c},sK:function(){return s}}),r(3299),r(1163);let i=e=>{let t=e=>fetch(e).then(e=>e.json());return console.log(Promise.all(e.map(t))),Promise.all(e.map(t))},c=e=>fetch(e).then(e=>e.json());async function a(e,t){var r=new URLSearchParams;Object.entries(t).forEach(e=>r.append(e[0],e[1]));try{return{ok:await (await fetch("http://143.198.77.99:8000"+"".concat(e),{method:"POST",headers:{"Content-Type":"application/x-www-form-urlencoded",credentials:"include",withCredentials:"true"},body:r})).text(),err:null}}catch(n){return{ok:null,err:n}}}async function s(e){var t=new URLSearchParams;Object.entries(e).forEach(e=>t.append(e[0],e[1])),console.log(t);try{let r=await fetch("http://143.198.77.99:8000/submit",{method:"POST",headers:{"Content-Type":"application/x-www-form-urlencoded",credentials:"include",withCredentials:"true"},body:t});if(200!==r.status){let n=await r.text();return{ok:null,err:n}}return{ok:await r.json(),err:null}}catch(l){return{ok:null,err:l}}}}},function(e){e.O(0,[299,734,774,888,179],function(){return e(e.s=2042)}),_N_E=e.O()}]);