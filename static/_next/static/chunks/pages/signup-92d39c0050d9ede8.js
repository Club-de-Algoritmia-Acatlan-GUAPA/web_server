(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[616],{7805:function(e,t,r){(window.__NEXT_P=window.__NEXT_P||[]).push(["/signup",function(){return r(3599)}])},2422:function(e,t,r){"use strict";r.d(t,{M:function(){return u}});var n=r(5893),a=r(4869),o=r(5675),s=r.n(o),c=r(7294),i=r(2459);let l=e=>{let{children:t,onClick:r=()=>{},color:o,enable:s=!0}=e,i=(0,c.useRef)(null);return(0,n.jsx)(n.Fragment,{children:(0,n.jsx)("button",{ref:i,onMouseDown:e=>{i.current&&s&&(i.current.style.background="var(--".concat(o,"-primary-faded)"))},onMouseUp:e=>{i.current&&(i.current.style.background="var(--".concat(o,"-primary)"))},onMouseOut:e=>{i.current&&(i.current.style.background="var(--".concat(o,"-primary)"))},style:{background:"var(--".concat(o,"-primary)"),border:"1px solid var(--".concat(o,"-secondary)"),borderRadius:"var(--border-radius)",width:"126px",padding:"0 10px 0 10px",height:"29px",cursor:"pointer"},onClick:s?r:()=>{},children:(0,n.jsx)(a.Z,{color:"white",type:"h4",children:t})})})},u=e=>{let{onClick:t,loading:r}=e;return(0,n.jsx)(n.Fragment,{children:(0,n.jsx)(l,{enable:!r,onClick:r?()=>{}:t,color:"blue",children:r?(0,n.jsx)(i.Z,{}):(0,n.jsxs)("div",{style:{display:"flex",alignItems:"center",justifyContent:"center",gap:"5px"},children:[(0,n.jsx)(s(),{width:18,height:18,alt:"submit logo",src:"/submit_plane.svg"}),"Submit"]})})})};t.Z=l},7226:function(e,t,r){"use strict";r.d(t,{Z:function(){return c}});var n=r(5893);let a={borderRadius:"10px",padding:"10px",background:"rgba(255, 0, 0, 0.3)"},o=e=>{let{children:t,key:r}=e;return(0,n.jsx)(n.Fragment,{children:(0,n.jsx)("div",{style:a,children:t},r)})},s=e=>{let{messages:t}=e,r=[];return"length"in t?r=[...t]:t.issues.forEach(e=>{r.push(e.message)}),(0,n.jsx)(n.Fragment,{children:r.length&&r.map(e=>(0,n.jsx)(o,{children:e},e))})};var c=s},3682:function(e,t,r){"use strict";var n=r(5893),a=r(5499),o=r.n(a),s=r(4869);let c=e=>{let{label:t,children:r,onSubmit:a,width:c="430px"}=e;return(0,n.jsx)(n.Fragment,{children:(0,n.jsxs)("form",{style:{width:c},className:o().form,onSubmit:a,children:[t&&(0,n.jsxs)(n.Fragment,{children:[(0,n.jsxs)("label",{children:[" ",(0,n.jsxs)(s.Z,{type:"h2",color:"primary",children:[" ",t," "]})," "]}),(0,n.jsx)("hr",{})]}),r]})})};t.Z=c},536:function(e,t,r){"use strict";var n=r(5893),a=r(4869),o=r(7294),s=r(883),c=r.n(s);let i=e=>{let{label:t,placeholder:r,text:s,id:i,type:l="text",onValidation:u,onChange:d}=e,[h,m]=(0,o.useState)();return(0,n.jsx)(n.Fragment,{children:(0,n.jsxs)("div",{className:c().container,children:[(0,n.jsxs)("label",{className:c().label,htmlFor:i,children:[" ",(0,n.jsx)(a.Z,{type:"h3",color:"primary",children:t})]}),(0,n.jsx)("input",{className:c().input,placeholder:r,id:i,type:l,min:"1",onChange:function(e){m(void 0),d&&d(e)},onSubmit:function(e){if(!u)return;let t=u(e.target.value);t.success?m(void 0):m(t.error)}}),s&&(0,n.jsxs)("label",{className:c().text,children:[" ",(0,n.jsx)(a.Z,{color:"primary",type:"text",sz:"sm",children:s})," "]})]})})};t.Z=i},2459:function(e,t,r){"use strict";var n=r(5893);let a=()=>(0,n.jsx)(n.Fragment,{children:(0,n.jsx)("div",{children:" Loading ...  "})});t.Z=a},6669:function(e,t,r){"use strict";r.d(t,{Z:function(){return o}});var n=r(5893);let a={flex:1,display:"flex",flexDirection:"row",justifyContent:"center",alignItems:"center"};function o(e){return(0,n.jsx)("main",{style:a,children:e})}},3599:function(e,t,r){"use strict";r.r(t);var n,a,o=r(5893),s=r(3682),c=r(6669),i=r(7294),l=r(536),u=r(2422),d=r(7226),h=r(1604),m=r(8241);let f=h.z.object({name:h.z.string({required_error:"Name is required"}).min(3,{message:"El nombre debe tener al menos 3 caracteres"}),email:h.z.string().max(100,{message:"El correo debe tener menos de 100 caracteres"}).email(),password:h.z.string().min(8).max(50),passwordConfirmation:h.z.string().min(8).max(50)}).superRefine((e,t)=>{let{passwordConfirmation:r,password:n}=e;r!==n&&t.addIssue({code:"custom",message:"Las contrase\xf1as no coinciden"})});(n=a||(a={}))[n.SET_EMAIL=0]="SET_EMAIL",n[n.SET_PASSWORD=1]="SET_PASSWORD",n[n.SET_PASSWORD_CONFIRMATION=2]="SET_PASSWORD_CONFIRMATION",n[n.SET_USERNAME=3]="SET_USERNAME";let p=()=>{let[e,t]=(0,i.useReducer)(function(e,t){switch(t.type){case a.SET_EMAIL:return{...e,email:t.email};case a.SET_PASSWORD:return{...e,password:t.password};case a.SET_PASSWORD_CONFIRMATION:return{...e,passwordConfirmation:t.passwordConfirmation};case a.SET_USERNAME:return{...e,name:t.name}}},{name:"",email:"",password:"",passwordConfirmation:""}),[r,n]=(0,i.useState)(),[c,h]=(0,i.useState)(!1);return(0,o.jsx)(o.Fragment,{children:(0,o.jsxs)(s.Z,{onSubmit:function(t){t.preventDefault();let r=f.safeParse(e);if(r.success){n(void 0);var a=new URLSearchParams;Object.entries(e).forEach(e=>a.append(e[0],e[1])),n(void 0),h(!0),(async()=>{(await (0,m.Md)("/signup",e)).ok?window.location.replace("/login"):n(["Something went wrong"])})(),h(!1)}else{let o=[];r.error.issues.forEach(e=>{o.push(e.message)}),n(r.error)}},label:"Crea tu cuenta",children:[(0,o.jsx)(l.Z,{label:"Username",id:"username",onChange:function(e){t({type:a.SET_USERNAME,name:e.target.value})},text:"El usuario no puede contener los siguientes caracteres"}),(0,o.jsx)(l.Z,{label:"E-mail",id:"email",onChange:function(e){t({type:a.SET_EMAIL,email:e.target.value})}}),(0,o.jsx)(l.Z,{label:"Contrase\xf1a",id:"password",type:"password",onChange:function(e){t({type:a.SET_PASSWORD,password:e.target.value})}}),(0,o.jsx)(l.Z,{label:"Confirma tu contrase\xf1a",id:"confirmPassword",type:"password",onChange:function(e){t({type:a.SET_PASSWORD_CONFIRMATION,passwordConfirmation:e.target.value})}}),(0,o.jsx)(u.Z,{enable:!c,color:"green",children:c?"Loading":"Crear"}),r&&(0,o.jsx)(d.Z,{messages:r})]})})};p.getLayout=c.Z,t.default=p},8241:function(e,t,r){"use strict";r.d(t,{C3:function(){return s},Md:function(){return c},Tq:function(){return o},ib:function(){return a},sK:function(){return l},ty:function(){return i}});var n=r(7294);let a=e=>{let[t,r]=(0,n.useState)(!1),[a,o]=(0,n.useState)(),[s,c]=(0,n.useState)(),i=()=>{fetch(encodeURI("http://localhost:8000"+e),{cache:"force-cache"}).then(e=>e.json()).then(e=>{o(""),c(e)}).catch(e=>o(e.message)).finally(()=>r(!1))};return(0,n.useEffect)(()=>{r(!0),i()},[]),[t,s,a]};async function o(e){try{let t=await fetch("".concat("http://localhost:8000","/submission-get?problem_id=").concat(e,"&from=0&to=0"));if(console.log(t),t.redirected)return window.location.replace(t.url),{ok:null,err:"redirected"};if(200!==t.status){let r=await t.text();return console.log(r),{ok:r,err:null}}let n=await t.json();return console.log("infoooooooooooo",n),{ok:n,err:null}}catch(a){return{ok:null,err:a}}}async function s(e){try{let t=await fetch(encodeURI("".concat("http://localhost:8000","/problem-get?id=").concat(e)),{cache:"force-cache"});if(console.log(t),t.redirected)return{ok:null,err:"redirected"};if(200!==t.status){let r=await t.text();console.log(r)}let n=await t.json();return console.log(n),{ok:n,err:null}}catch(a){return{ok:null,err:a}}}async function c(e,t){var r=new URLSearchParams;Object.entries(t).forEach(e=>r.append(e[0],e[1]));try{return{ok:await (await fetch("http://localhost:8000"+"".concat(e),{method:"POST",headers:{"Content-Type":"application/x-www-form-urlencoded",credentials:"include",withCredentials:"true"},body:r})).text(),err:null}}catch(n){return{ok:null,err:n}}}async function i(e,t){try{return{ok:await (await fetch("http://localhost:8000"+"".concat(e),{method:"POST",headers:{"Content-Type":"application/json",credentials:"include",withCredentials:"true"},body:JSON.stringify(t)})).json(),err:null}}catch(r){return{ok:null,err:r}}}async function l(e){var t=new URLSearchParams;let r=new FormData;Object.entries(e).forEach(e=>r.append(e[0],e[1])),console.log(t);try{let n=await fetch("http://localhost:8000/submit",{method:"POST",headers:{credentials:"include",withCredentials:"true"},body:r});if(200!==n.status){let a=await n.text();return{ok:null,err:a}}return{ok:await n.json(),err:null}}catch(o){return{ok:null,err:o}}}},5499:function(e){e.exports={label:"Form_label__QVfSm",form:"Form_form__mh4_m"}},883:function(e){e.exports={input:"Input_input__2jWMJ",container:"Input_container__B9ERX",label:"Input_label__o6r7L",text:"Input_text__GBVsM"}}},function(e){e.O(0,[604,774,888,179],function(){return e(e.s=7805)}),_N_E=e.O()}]);