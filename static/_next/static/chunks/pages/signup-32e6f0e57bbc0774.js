(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[616],{7805:function(e,r,t){(window.__NEXT_P=window.__NEXT_P||[]).push(["/signup",function(){return t(3599)}])},2422:function(e,r,t){"use strict";t.d(r,{M:function(){return u}});var n=t(5893),a=t(4869),o=t(5675),s=t.n(o),l=t(7294),i=t(2459);let c=e=>{let{children:r,onClick:t=()=>{},color:o,enable:s=!0}=e,i=(0,l.useRef)(null);return(0,n.jsx)(n.Fragment,{children:(0,n.jsx)("button",{ref:i,onMouseDown:e=>{i.current&&s&&(i.current.style.background="var(--blue-primary-faded)")},onMouseUp:e=>{i.current&&(i.current.style.background="var(--blue-primary)")},onMouseOut:e=>{i.current&&(i.current.style.background="var(--blue-primary)")},style:{background:"var(--".concat(o,"-primary)"),border:"1px solid var(--".concat(o,"-secondary)"),borderRadius:"var(--border-radius)",width:"126px",padding:"0 10px 0 10px",height:"29px"},onClick:s?t:()=>{},children:(0,n.jsx)(a.Z,{color:"white",type:"h4",children:r})})})},u=e=>{let{onClick:r,loading:t}=e;return(0,n.jsx)(n.Fragment,{children:(0,n.jsx)(c,{enable:!t,onClick:t?()=>{}:r,color:"blue",children:t?(0,n.jsx)(i.Z,{}):(0,n.jsxs)("div",{style:{display:"flex",alignItems:"center",justifyContent:"center",gap:"5px"},children:[(0,n.jsx)(s(),{width:18,height:18,alt:"submit logo",src:"/submit_plane.svg"}),"Submit"]})})})};r.Z=c},7226:function(e,r,t){"use strict";t.d(r,{Z:function(){return l}});var n=t(5893);let a={borderRadius:"10px",padding:"10px",background:"rgba(255, 0, 0, 0.3)"},o=e=>{let{children:r,key:t}=e;return(0,n.jsx)(n.Fragment,{children:(0,n.jsx)("div",{style:a,children:r},t)})},s=e=>{let{messages:r}=e,t=[];return"length"in r?t=[...r]:r.issues.forEach(e=>{t.push(e.message)}),(0,n.jsx)(n.Fragment,{children:t.length&&t.map(e=>(0,n.jsx)(o,{children:e},e))})};var l=s},3682:function(e,r,t){"use strict";var n=t(5893),a=t(5499),o=t.n(a),s=t(4869);let l=e=>{let{label:r,children:t,onSubmit:a}=e;return(0,n.jsx)(n.Fragment,{children:(0,n.jsxs)("form",{className:o().form,onSubmit:a,children:[(0,n.jsxs)("label",{children:[" ",(0,n.jsxs)(s.Z,{type:"h2",color:"primary",children:[" ",r," "]})," "]}),(0,n.jsx)("hr",{}),t]})})};r.Z=l},536:function(e,r,t){"use strict";var n=t(5893),a=t(4869),o=t(7294),s=t(883),l=t.n(s);let i=e=>{let{label:r,placeholder:t,text:s,id:i,type:c="text",onValidation:u,onChange:d}=e,[m,p]=(0,o.useState)();return(0,n.jsx)(n.Fragment,{children:(0,n.jsxs)("div",{className:l().container,children:[(0,n.jsxs)("label",{className:l().label,htmlFor:i,children:[" ",(0,n.jsx)(a.Z,{type:"h3",color:"primary",children:r})]}),(0,n.jsx)("input",{className:l().input,placeholder:t,id:i,type:c,onChange:function(e){p(void 0),d&&d(e)},onSubmit:function(e){if(!u)return;let r=u(e.target.value);r.success?p(void 0):(console.log("adas"),p(r.error))}}),s&&(0,n.jsxs)("label",{className:l().text,children:[" ",(0,n.jsx)(a.Z,{color:"primary",type:"text",sz:"sm",children:s})," "]})]})})};r.Z=i},2459:function(e,r,t){"use strict";var n=t(5893);let a=()=>(0,n.jsx)(n.Fragment,{children:(0,n.jsx)("div",{children:" Loading ...  "})});r.Z=a},6669:function(e,r,t){"use strict";t.d(r,{Z:function(){return o}});var n=t(5893);let a={flex:1,display:"flex","flex-direction":"row",justifyContent:"center",alignItems:"center"};function o(e){return(0,n.jsx)("main",{style:a,children:e})}},3599:function(e,r,t){"use strict";t.r(r);var n,a,o=t(5893),s=t(3682),l=t(6669),i=t(7294),c=t(536),u=t(2422),d=t(7226),m=t(1604),p=t(8241);let h=m.z.object({name:m.z.string({required_error:"Name is required"}).min(3,{message:"El nombre debe tener al menos 3 caracteres"}),email:m.z.string().max(100,{message:"El correo debe tener menos de 100 caracteres"}).email(),password:m.z.string().min(8).max(50),passwordConfirmation:m.z.string().min(8).max(50)}).superRefine((e,r)=>{let{passwordConfirmation:t,password:n}=e;t!==n&&r.addIssue({code:"custom",message:"Las contrase\xf1as no coinciden"})});function f(e,r){switch(r.type){case a.SET_EMAIL:return{...e,email:r.email};case a.SET_PASSWORD:return{...e,password:r.password};case a.SET_PASSWORD_CONFIRMATION:return{...e,passwordConfirmation:r.passwordConfirmation};case a.SET_USERNAME:return{...e,name:r.name}}}(n=a||(a={}))[n.SET_EMAIL=0]="SET_EMAIL",n[n.SET_PASSWORD=1]="SET_PASSWORD",n[n.SET_PASSWORD_CONFIRMATION=2]="SET_PASSWORD_CONFIRMATION",n[n.SET_USERNAME=3]="SET_USERNAME";let x=()=>{let[e,r]=(0,i.useReducer)(f,{name:"",email:"",password:"",passwordConfirmation:""}),[t,n]=(0,i.useState)(),[l,m]=(0,i.useState)(!1);return(0,o.jsx)(o.Fragment,{children:(0,o.jsxs)(s.Z,{onSubmit:function(r){r.preventDefault();let t=h.safeParse(e);if(t.success){n(void 0);var a=new URLSearchParams;Object.entries(e).forEach(e=>a.append(e[0],e[1])),console.log(a),n(void 0),m(!0),(async()=>{let r=await (0,p.Md)("/signup",e);r.ok?window.location.replace("/login"):console.log(r.err)})(),m(!1)}else{let o=[];t.error.issues.forEach(e=>{o.push(e.message)}),n(t.error)}},label:"Crea tu cuenta",children:[(0,o.jsx)(c.Z,{label:"Username",id:"username",onChange:function(e){r({type:a.SET_USERNAME,name:e.target.value})},text:"El usuario no puede contener los siguientes caracteres"}),(0,o.jsx)(c.Z,{label:"E-mail",id:"email",onChange:function(e){r({type:a.SET_EMAIL,email:e.target.value})}}),(0,o.jsx)(c.Z,{label:"Contrase\xf1a",id:"password",type:"password",onChange:function(e){r({type:a.SET_PASSWORD,password:e.target.value})}}),(0,o.jsx)(c.Z,{label:"Confirma tu contrase\xf1a",id:"confirmPassword",type:"password",onChange:function(e){r({type:a.SET_PASSWORD_CONFIRMATION,passwordConfirmation:e.target.value})}}),(0,o.jsx)(u.Z,{enable:l,color:"green",children:"Crear"}),t&&(0,o.jsx)(d.Z,{messages:t})]})})};x.getLayout=l.Z,r.default=x},8241:function(e,r,t){"use strict";async function n(e){try{let r=await fetch("".concat("http://localhost:8000","/submission-get?problem_id=").concat(e,"&from=0&to=0"),{headers:{credentials:"include",withCredentials:"true"}});if(console.log(r),r.redirected)return window.location.replace(r.url),{ok:null,err:"redirected"};if(200!==r.status){let t=await r.text();return console.log(t),{ok:t,err:null}}let n=await r.json();return console.log("infoooooooooooo",n),{ok:n,err:null}}catch(a){return{ok:null,err:a}}}async function a(e){try{let r=await fetch("".concat("http://localhost:8000","/problem-get?id=").concat(e),{cache:"force-cache"});if(console.log(r),r.redirected)return window.location.replace(r.url),{ok:null,err:"redirected"};if(200!==r.status){let t=await r.text();console.log(t)}let n=await r.json();return console.log(n),{ok:n,err:null}}catch(a){return window.location.replace("/404"),{ok:null,err:a}}}t.d(r,{C3:function(){return a},Md:function(){return l},Tq:function(){return n},XH:function(){return o},_i:function(){return s},sK:function(){return i}}),t(3299),t(1163);let o=e=>{let r=e=>fetch(e).then(e=>e.json());return console.log(Promise.all(e.map(r))),Promise.all(e.map(r))},s=e=>fetch(e).then(e=>e.json());async function l(e,r){var t=new URLSearchParams;Object.entries(r).forEach(e=>t.append(e[0],e[1]));try{return{ok:await (await fetch("http://localhost:8000"+"".concat(e),{method:"POST",headers:{"Content-Type":"application/x-www-form-urlencoded",credentials:"include",withCredentials:"true"},body:t})).text(),err:null}}catch(n){return{ok:null,err:n}}}async function i(e){var r=new URLSearchParams;Object.entries(e).forEach(e=>r.append(e[0],e[1])),console.log(r);try{let t=await fetch("http://localhost:8000/submit",{method:"POST",headers:{"Content-Type":"application/x-www-form-urlencoded",credentials:"include",withCredentials:"true"},body:r});if(200!==t.status){let n=await t.text();return{ok:null,err:n}}return{ok:await t.json(),err:null}}catch(a){return{ok:null,err:a}}}},5499:function(e){e.exports={label:"Form_label__QVfSm",form:"Form_form__mh4_m"}},883:function(e){e.exports={input:"Input_input__2jWMJ",container:"Input_container__B9ERX",label:"Input_label__o6r7L",text:"Input_text__GBVsM"}}},function(e){e.O(0,[299,604,774,888,179],function(){return e(e.s=7805)}),_N_E=e.O()}]);