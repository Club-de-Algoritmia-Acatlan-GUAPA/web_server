(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[308],{3431:function(e,t,r){"use strict";r.d(t,{Z:function(){return o}});var i=r(655),n=r(7294),s=r(16),a=r(5403),l=r(1539);function o(e){var{variant:t="div",margin:r={},padding:s={}}=e,o=(0,i._T)(e,["variant","margin","padding"]);let c=(0,l.Z)("Box",{props:{color:o.color,display:o.display,float:o.float,fontSize:o.fontSize,fontWeight:o.fontWeight,textAlign:o.textAlign,variant:t}});return n.createElement(a.Z,Object.assign({variant:t,margin:r,padding:s},o,c))}(0,s.b)(o,"Box")},6787:function(e,t,r){"use strict";r.d(t,{Z:function(){return p}});var i=r(655),n=r(997),s=r(7294),a=r(410),l=r(2151),o=r(709),c=r(9495);r(9329);var u={root:"awsui_root_k2y2q_1wk1c_97","checkbox-control":"awsui_checkbox-control_k2y2q_1wk1c_133",outline:"awsui_outline_k2y2q_1wk1c_141"},d=r(3766),h=r(8525),_=r(5858);let f=s.forwardRef((e,t)=>{var{controlId:r,name:f,checked:p,disabled:w,readOnly:g,ariaRequired:m,indeterminate:y,children:v,description:b,ariaLabel:k,onFocus:x,onBlur:j,onChange:R,tabIndex:E,showOutline:C,ariaControls:O,__internalRootRef:I}=e,Z=(0,i._T)(e,["controlId","name","checked","disabled","readOnly","ariaRequired","indeterminate","children","description","ariaLabel","onFocus","onBlur","onChange","tabIndex","showOutline","ariaControls","__internalRootRef"]);let{ariaDescribedby:S,ariaLabelledby:Q}=(0,h.e)(Z),q=(0,o.j)(Z),T=(0,s.useRef)(null);(0,l.Z)(t,T),(0,s.useEffect)(()=>{T.current&&(T.current.indeterminate=Boolean(y))});let{tabIndex:N}=(0,_.s_)(T,{tabIndex:E});return s.createElement(c.Z,Object.assign({},q,{className:(0,n.Z)(u.root,q.className),controlClassName:u["checkbox-control"],outlineClassName:u.outline,controlId:r,disabled:w,readOnly:g,label:v,description:b,descriptionBottomPadding:!0,ariaLabel:k,ariaLabelledby:Q,ariaDescribedby:S,ariaControls:O,showOutline:C,nativeControl:e=>s.createElement("input",Object.assign({},e,{ref:T,type:"checkbox",checked:p,name:f,"aria-required":m?"true":void 0,"aria-disabled":g&&!w?"true":void 0,tabIndex:N,onFocus:()=>(0,a.B4)(x),onBlur:()=>(0,a.B4)(j),onChange:()=>{}})),onClick:()=>{var e;null===(e=T.current)||void 0===e||e.focus(),(0,a.B4)(R,y?{checked:!0,indeterminate:!1}:{checked:!p,indeterminate:!1})},styledControl:s.createElement(d.Z,{checked:p,indeterminate:y,disabled:w,readOnly:g}),__internalRootRef:I}))});var p=f},9021:function(e,t,r){"use strict";r.d(t,{Z:function(){return o}});var i=r(655),n=r(7294),s=r(16),a=r(9835),l=r(1539);function o(e){var{variant:t="h2"}=e,r=(0,i._T)(e,["variant"]);let s=(0,l.Z)("Header",{props:{headingTagOverride:r.headingTagOverride,variant:t}});return n.createElement(a.Z,Object.assign({variant:t},r,s))}(0,s.b)(o,"Header")},3766:function(e,t,r){"use strict";r.d(t,{Z:function(){return d}});var i=r(655),n=r(997),s=r(7294),a=r(709),l=r(1253);r(9383);var o={root:"awsui_root_1fn7j_1yjhw_97","styled-box":"awsui_styled-box_1fn7j_1yjhw_104","styled-box-checked":"awsui_styled-box-checked_1fn7j_1yjhw_110","styled-box-indeterminate":"awsui_styled-box-indeterminate_1fn7j_1yjhw_110","styled-box-disabled":"awsui_styled-box-disabled_1fn7j_1yjhw_114","styled-box-readonly":"awsui_styled-box-readonly_1fn7j_1yjhw_114","styled-line":"awsui_styled-line_1fn7j_1yjhw_128","styled-line-disabled":"awsui_styled-line-disabled_1fn7j_1yjhw_133","styled-line-readonly":"awsui_styled-line-readonly_1fn7j_1yjhw_136"};let c={default:{viewBox:"0 0 14 14",indeterminate:"2.5,7 11.5,7",checked:"2.5,7 6,10 11,3",xy:.5,r:1.5,size:13},refresh:{viewBox:"0 0 16 16",indeterminate:"3.5,8 12.5,8",checked:"3.5,8 7,11 12,4",xy:1,r:2,size:14}},u=e=>{var{checked:t,indeterminate:r,disabled:u=!1,readOnly:d=!1}=e,h=(0,i._T)(e,["checked","indeterminate","disabled","readOnly"]);let _=(0,a.j)(h),f=(0,l.L)()?"refresh":"default",p=c[f];return s.createElement("svg",Object.assign({className:o.root,viewBox:p.viewBox,"aria-hidden":"true",focusable:"false"},_),s.createElement("rect",{className:(0,n.Z)(o["styled-box"],{[o["styled-box-checked"]]:t,[o["styled-box-indeterminate"]]:r,[o["styled-box-disabled"]]:u,[o["styled-box-readonly"]]:d}),x:p.xy,y:p.xy,rx:p.r,ry:p.r,width:p.size,height:p.size}),t||r?s.createElement("polyline",{className:(0,n.Z)(o["styled-line"],{[o["styled-line-disabled"]]:u,[o["styled-line-readonly"]]:d}),points:r?p.indeterminate:p.checked}):null)};var d=u},5951:function(e,t,r){"use strict";r.d(t,{Z:function(){return a}});var i=r(9024),n=r(7294);let s=(0,i._N)(e=>{function t(t){e(t.target)}function r(){e(null)}return window.addEventListener("mousedown",t),window.addEventListener("keydown",r),()=>{window.removeEventListener("mousedown",t),window.removeEventListener("keydown",r)}});function a(){let e=(0,n.useRef)(null);return s(t=>{e.current=t}),()=>e.current}},3361:function(e,t,r){"use strict";var i=r(655),n=r(7294),s=r(1539),a=r(16),l=r(3166);let o=n.forwardRef((e,t)=>{var{fontSize:r="body-m",color:a="normal",external:o=!1}=e,c=(0,i._T)(e,["fontSize","color","external"]);let u=(0,s.Z)("Link",{props:{color:a,external:o,fontSize:r,rel:c.rel,target:c.target,variant:c.variant}});return n.createElement(l.Z,Object.assign({fontSize:r,color:a,external:o},c,u,{ref:t}))});(0,a.b)(o,"Link"),t.Z=o},3564:function(e,t,r){"use strict";r.d(t,{Z:function(){return c}});var i=r(655),n=r(997),s=r(7294),a=r(709);r(553);var l={root:"awsui_root_18582_jftqg_97",child:"awsui_child_18582_jftqg_101",horizontal:"awsui_horizontal_18582_jftqg_112","horizontal-xxxs":"awsui_horizontal-xxxs_18582_jftqg_116","horizontal-xxs":"awsui_horizontal-xxs_18582_jftqg_119","horizontal-xs":"awsui_horizontal-xs_18582_jftqg_122","horizontal-s":"awsui_horizontal-s_18582_jftqg_125","horizontal-m":"awsui_horizontal-m_18582_jftqg_128","horizontal-l":"awsui_horizontal-l_18582_jftqg_131","horizontal-xl":"awsui_horizontal-xl_18582_jftqg_134","horizontal-xxl":"awsui_horizontal-xxl_18582_jftqg_137",vertical:"awsui_vertical_18582_jftqg_144","vertical-xxxs":"awsui_vertical-xxxs_18582_jftqg_147","vertical-xxs":"awsui_vertical-xxs_18582_jftqg_150","vertical-xs":"awsui_vertical-xs_18582_jftqg_153","vertical-s":"awsui_vertical-s_18582_jftqg_156","vertical-m":"awsui_vertical-m_18582_jftqg_159","vertical-l":"awsui_vertical-l_18582_jftqg_162","vertical-xl":"awsui_vertical-xl_18582_jftqg_165","vertical-xxl":"awsui_vertical-xxl_18582_jftqg_168","align-center":"awsui_align-center_18582_jftqg_172","align-start":"awsui_align-start_18582_jftqg_176","align-end":"awsui_align-end_18582_jftqg_180"},o=r(3262);function c(e){var{direction:t="vertical",size:r,children:c,alignItems:u,__internalRootRef:d}=e,h=(0,i._T)(e,["direction","size","children","alignItems","__internalRootRef"]);let _=(0,a.j)(h),f=(0,o.Z)(c);return s.createElement("div",Object.assign({},_,{className:(0,n.Z)(_.className,l.root,l[t],l["".concat(t,"-").concat(r)],u&&l["align-".concat(u)]),ref:d}),f.map(e=>{let t="object"==typeof e?e.key:void 0;return s.createElement("div",{key:t,className:l.child},e)}))}},8151:function(e,t,r){"use strict";r.d(t,{Z:function(){return d}});var i=r(655),n=r(7294),s=r(997),a=r(709);r(1527);var l={"icon-shake":"awsui_icon-shake_1cbgc_pjowk_101","awsui-motion-shake-horizontally":"awsui_awsui-motion-shake-horizontally_1cbgc_pjowk_1","container-fade-in":"awsui_container-fade-in_1cbgc_pjowk_129","awsui-motion-fade-in-0":"awsui_awsui-motion-fade-in-0_1cbgc_pjowk_1",root:"awsui_root_1cbgc_pjowk_151","status-error":"awsui_status-error_1cbgc_pjowk_160","status-warning":"awsui_status-warning_1cbgc_pjowk_163","status-success":"awsui_status-success_1cbgc_pjowk_166","status-info":"awsui_status-info_1cbgc_pjowk_169","status-stopped":"awsui_status-stopped_1cbgc_pjowk_172","status-pending":"awsui_status-pending_1cbgc_pjowk_175","status-in-progress":"awsui_status-in-progress_1cbgc_pjowk_178","status-loading":"awsui_status-loading_1cbgc_pjowk_181","color-override-red":"awsui_color-override-red_1cbgc_pjowk_184","color-override-grey":"awsui_color-override-grey_1cbgc_pjowk_187","color-override-blue":"awsui_color-override-blue_1cbgc_pjowk_190","color-override-green":"awsui_color-override-green_1cbgc_pjowk_193","color-override-yellow":"awsui_color-override-yellow_1cbgc_pjowk_196",container:"awsui_container_1cbgc_pjowk_129","display-inline":"awsui_display-inline_1cbgc_pjowk_200",icon:"awsui_icon_1cbgc_pjowk_101","display-inline-block":"awsui_display-inline-block_1cbgc_pjowk_208","overflow-ellipsis":"awsui_overflow-ellipsis_1cbgc_pjowk_217"},o=r(294),c=r(2609);let u=e=>({error:n.createElement(o.Z,{name:"status-negative",size:e}),warning:n.createElement(o.Z,{name:"status-warning",size:e}),success:n.createElement(o.Z,{name:"status-positive",size:e}),info:n.createElement(o.Z,{name:"status-info",size:e}),stopped:n.createElement(o.Z,{name:"status-stopped",size:e}),pending:n.createElement(o.Z,{name:"status-pending",size:e}),"in-progress":n.createElement(o.Z,{name:"status-in-progress",size:e}),loading:n.createElement(c.Z,null)});function d(e){var{type:t,children:r,iconAriaLabel:o,colorOverride:c,wrapText:d=!0,__animate:h=!1,__internalRootRef:_,__size:f="normal",__display:p="inline-block"}=e,w=(0,i._T)(e,["type","children","iconAriaLabel","colorOverride","wrapText","__animate","__internalRootRef","__size","__display"]);let g=(0,a.j)(w);return n.createElement("span",Object.assign({},g,{className:(0,s.Z)(l.root,l["status-".concat(t)],{[l["color-override-".concat(c)]]:c},g.className),ref:_}),n.createElement("span",{className:(0,s.Z)(l.container,l["display-".concat(p)],!1===d&&l["overflow-ellipsis"],h&&l["container-fade-in"])},n.createElement("span",{className:(0,s.Z)(l.icon,h&&l["icon-shake"]),"aria-label":o,role:o?"img":void 0},u(f)[t],"inline"===p&&n.createElement(n.Fragment,null,"\xa0")),r))}},3079:function(e,t,r){"use strict";r.d(t,{c:function(){return g}});var i=r(655),n=r(997),s=r(7294),a=r(2592),l=r(4171),o=r(6787),c=r(9495),u=r(410),d=r(9787),h=r(1253);r(5300);var _={root:"awsui_root_1mabk_fodq7_97",radio:"awsui_radio_1mabk_fodq7_133","radio--has-description":"awsui_radio--has-description_1mabk_fodq7_141","radio-control":"awsui_radio-control_1mabk_fodq7_145",outline:"awsui_outline_1mabk_fodq7_153","styled-circle-border":"awsui_styled-circle-border_1mabk_fodq7_175","styled-circle-disabled":"awsui_styled-circle-disabled_1mabk_fodq7_179","styled-circle-readonly":"awsui_styled-circle-readonly_1mabk_fodq7_179","styled-circle-fill":"awsui_styled-circle-fill_1mabk_fodq7_184","styled-circle-checked":"awsui_styled-circle-checked_1mabk_fodq7_190"},f=r(5858),p=s.forwardRef(function(e,t){let{name:r,label:i,value:a,checked:l,description:o,disabled:p,controlId:w,onChange:g,readOnly:m}=e,y=(0,h.L)(),v=(0,s.useRef)(null),b=(0,d.q)(v,t),{tabIndex:k}=(0,f.s_)(v);return s.createElement(c.Z,{className:(0,n.Z)(_.radio,o&&_["radio--has-description"]),controlClassName:_["radio-control"],outlineClassName:_.outline,label:i,description:o,disabled:p,readOnly:m,controlId:w,nativeControl:e=>s.createElement("input",Object.assign({},e,{tabIndex:k,type:"radio",ref:b,name:r,value:a,checked:l,"aria-disabled":m&&!p?"true":void 0,onChange:()=>{}})),onClick:()=>{var e;null===(e=v.current)||void 0===e||e.focus(),l||(0,u.B4)(g,{value:a})},styledControl:s.createElement("svg",{viewBox:"0 0 100 100",focusable:"false","aria-hidden":"true"},s.createElement("circle",{className:(0,n.Z)(_["styled-circle-border"],{[_["styled-circle-disabled"]]:p,[_["styled-circle-readonly"]]:m}),strokeWidth:y?12:8,cx:50,cy:50,r:y?44:46}),s.createElement("circle",{className:(0,n.Z)(_["styled-circle-fill"],{[_["styled-circle-disabled"]]:p,[_["styled-circle-checked"]]:l,[_["styled-circle-readonly"]]:m}),strokeWidth:30,cx:50,cy:50,r:35}))})}),w=r(1449);function g(e){var{selectionType:t,indeterminate:r=!1,onShiftToggle:c,onFocusUp:u,onFocusDown:d,name:h,ariaLabel:_,focusedComponent:g}=e,m=(0,i._T)(e,["selectionType","indeterminate","onShiftToggle","onFocusUp","onFocusDown","name","ariaLabel","focusedComponent"]);let y=(0,l.L)(),v="multi"===t,{navigationActive:b}=(0,s.useContext)(f.UO),k=e=>{v&&c&&c(e.shiftKey)},x=e=>{k(e),v&&e.preventDefault()},j=e=>{k(e),v&&!b&&(e.keyCode===a.V.up&&(e.preventDefault(),u&&u(e)),e.keyCode===a.V.down&&(e.preventDefault(),d&&d(e)))},R=e=>{let t=e.currentTarget,r="INPUT"===t.tagName?t:t.querySelector("input");null==r||r.focus()},E=v?s.createElement(o.Z,Object.assign({},m,{showOutline:"selection-control"===g,controlId:y,"data-focus-id":"selection-control",indeterminate:r})):s.createElement(p,Object.assign({},m,{controlId:y,name:h,value:"",label:""}));return s.createElement(s.Fragment,null,s.createElement("label",{onKeyDown:j,onKeyUp:k,onMouseDown:x,onMouseUp:k,onClick:R,htmlFor:y,className:(0,n.Z)(w.Z.label,w.Z.root),"aria-label":_,title:_},E),s.createElement("span",{className:w.Z.stud,"aria-hidden":!0},"\xa0"))}},1449:function(e,t,r){"use strict";r(7085),t.Z={root:"awsui_root_1s55x_svuwr_97",label:"awsui_label_1s55x_svuwr_101",stud:"awsui_stud_1s55x_svuwr_115"}},5436:function(e,t,r){"use strict";r.d(t,{A:function(){return a}});var i=r(4478),n=r(1449),s=r(2471);function a(e,t){if("multi"!==e)return{};function r(e,r,a){var l;let o=r,c=(0,i.jX)(e,e=>"true"===e.dataset.selectionRoot);for(;o>=-1&&o<t;){o+=a;let u=-1===(l=o)?c.querySelector("[data-".concat(s.MW,'="all"] .').concat(n.Z.root," input")):c.querySelectorAll("[data-".concat(s.MW,'="item"] .').concat(n.Z.root," input"))[l];if(u&&!u.disabled){u.focus();break}}}let[a,l]=[1,-1].map(e=>t=>{let n=t.currentTarget,s=(0,i.jX)(n,e=>"item"===e.dataset.selectionItem),a=Array.prototype.indexOf.call(s.parentElement.children,s);r(n,a,e)});return{moveFocusDown:a,moveFocusUp:l,moveFocus:r}}},2973:function(e,t,r){"use strict";r.d(t,{c:function(){return c}});var i=r(7294),n=r(410),s=r(4171),a=r(4705),l=r(395),o=r(2471);function c(e){let t=function(e){let{ariaLabels:t,isItemDisabled:r=()=>!1,onSelectionChange:i,selectedItems:a=[],selectionType:c,trackBy:u}=e,d=(0,s.L)();if("single"!==c)return{isItemSelected:()=>!1};let h=new o.iP(u,a.slice(0,1)),_=h.has.bind(h),f=e=>{r(e)||_(e)||(0,n.B4)(i,{selectedItems:[e]})};return{isItemSelected:_,getItemSelectionProps:e=>{var i;return{name:d,selectionType:"single",disabled:r(e),checked:_(e),onChange:()=>f(e),ariaLabel:(0,l.M)(null==t?void 0:t.selectionGroupLabel,null===(i=null==t?void 0:t.itemSelectionLabel)||void 0===i?void 0:i.call(t,{selectedItems:a},e))}}}}(e),r=function(e){let{ariaLabels:t,isItemDisabled:r=()=>!1,items:c,loading:u,onSelectionChange:d,selectedItems:h=[],selectionType:_,trackBy:f}=e,p=(0,s.L)(),[w,g]=(0,i.useState)(!1),[m,y]=(0,i.useState)(null);if("multi"!==_)return{isItemSelected:()=>!1};let v=new o.iP(f,h),b=v.has.bind(v),k=!0,x=!0;for(let j of c)k=k&&r(j),x=x&&(b(j)||r(j));let R=h.length>0&&x,E=h.length>0&&!x,C=new Map;c.forEach((e,t)=>C.set((0,a.mU)(f,e),t));let O=e=>{let t=m?C.get((0,a.mU)(f,m)):void 0;if(void 0!==t){let r=C.get((0,a.mU)(f,e));return c.slice(Math.min(r,t),Math.max(r,t)+1)}return[e]},I=e=>{let t=[...h];return e.forEach(e=>{b(e)||r(e)||t.push(e)}),t},Z=e=>{let t=new o.iP(f,e),i=[];return h.forEach(e=>{let n=t.has(e);(!n||r(e))&&i.push(e)}),i},S=()=>{let e=x?Z(c):I(c);(0,n.B4)(d,{selectedItems:e})},Q=e=>{if(!r(e)){let t=w?O(e):[e],i=b(e)?Z(t):I(t);(0,n.B4)(d,{selectedItems:i}),y(e)}};return{isItemSelected:b,getSelectAllProps:()=>{var e;return{name:p,selectionType:"multi",disabled:k||!!u,checked:R,indeterminate:E,onChange:S,ariaLabel:(0,l.M)(null==t?void 0:t.selectionGroupLabel,null===(e=null==t?void 0:t.allItemsSelectionLabel)||void 0===e?void 0:e.call(t,{selectedItems:h}))}},getItemSelectionProps:e=>{var i;return{name:p,selectionType:"multi",disabled:r(e),checked:b(e),onChange:()=>Q(e),onShiftToggle:e=>g(e),ariaLabel:(0,l.M)(null==t?void 0:t.selectionGroupLabel,null===(i=null==t?void 0:t.itemSelectionLabel)||void 0===i?void 0:i.call(t,{selectedItems:h},e))}}}}(e);return"single"===e.selectionType?t:r}},2471:function(e,t,r){"use strict";r.d(t,{MW:function(){return n},gC:function(){return a},iP:function(){return s}});var i=r(4705);let n="selection-item";class s{constructor(e,t){this.map=new Map,this.put=e=>this.map.set.call(this.map,(0,i.mU)(this.trackBy,e),e),this.has=e=>this.map.has.call(this.map,(0,i.mU)(this.trackBy,e)),this.forEach=this.map.forEach.bind(this.map),this.trackBy=e,t.forEach(this.put)}}let a={item:{["data-"+n]:"item"},all:{["data-"+n]:"all"},root:{"data-selection-root":"true"}}},9204:function(e,t,r){"use strict";r.d(t,{WU:function(){return l},ZP:function(){return s},cV:function(){return a}});var i=r(7846),n=r(9024);function s(e,t){let r=()=>{if(!e.current||!t.current)return;let r=a(e.current,t.current);r>0&&l(r,e.current)},i=r=>{if(!r||!e.current||!t.current)return;let i=(0,n.tI)(t.current).insetBlockEnd,s=i-(0,n.tI)(r).insetBlockStart;s>0&&l(s,e.current)};return{scrollToTop:r,scrollToItem:i}}function a(e,t){let r=(0,n.tI)(t),i=(0,n.tI)(e);return r.insetBlockStart-i.insetBlockStart}function l(e,t){let r=(0,i.Et)(t);r.length?r[0].scrollTop-=e:window.scrollTo({top:window.pageYOffset-e})}},310:function(e,t,r){"use strict";r(1869),t.Z={root:"awsui_root_wih1l_rg19l_101",tools:"awsui_tools_wih1l_rg19l_112","tools-filtering":"awsui_tools-filtering_wih1l_rg19l_120","tools-align-right":"awsui_tools-align-right_wih1l_rg19l_134","tools-pagination":"awsui_tools-pagination_wih1l_rg19l_138","tools-preferences":"awsui_tools-preferences_wih1l_rg19l_138","tools-small":"awsui_tools-small_wih1l_rg19l_144",table:"awsui_table_wih1l_rg19l_150","table-layout-fixed":"awsui_table-layout-fixed_wih1l_rg19l_156",wrapper:"awsui_wrapper_wih1l_rg19l_160","variant-stacked":"awsui_variant-stacked_wih1l_rg19l_167","wrapper-content-measure":"awsui_wrapper-content-measure_wih1l_rg19l_167","variant-container":"awsui_variant-container_wih1l_rg19l_167","has-footer":"awsui_has-footer_wih1l_rg19l_170","has-header":"awsui_has-header_wih1l_rg19l_173","cell-merged":"awsui_cell-merged_wih1l_rg19l_190","cell-merged-content":"awsui_cell-merged-content_wih1l_rg19l_202",empty:"awsui_empty_wih1l_rg19l_218",loading:"awsui_loading_wih1l_rg19l_222","selection-control":"awsui_selection-control_wih1l_rg19l_231","selection-control-header":"awsui_selection-control-header_wih1l_rg19l_238","header-secondary":"awsui_header-secondary_wih1l_rg19l_244","variant-full-page":"awsui_variant-full-page_wih1l_rg19l_256","table-has-header":"awsui_table-has-header_wih1l_rg19l_265","header-controls":"awsui_header-controls_wih1l_rg19l_269","variant-embedded":"awsui_variant-embedded_wih1l_rg19l_278","variant-borderless":"awsui_variant-borderless_wih1l_rg19l_278","footer-wrapper":"awsui_footer-wrapper_wih1l_rg19l_283",footer:"awsui_footer_wih1l_rg19l_283","footer-with-pagination":"awsui_footer-with-pagination_wih1l_rg19l_291","footer-pagination":"awsui_footer-pagination_wih1l_rg19l_299","thead-active":"awsui_thead-active_wih1l_rg19l_303",row:"awsui_row_wih1l_rg19l_304","row-selected":"awsui_row-selected_wih1l_rg19l_305"}},8434:function(e,t,r){"use strict";r.d(t,{Z:function(){return c}});var i=r(997),n=r(7294),s=r(1248),a=r(310),l=r(9629),o=r(4171);function c(e){let{header:t,filter:r,pagination:c,preferences:u,setLastUserAction:d}=e,[h,_]=(0,s.d)(["xs"]),f="string"==typeof t,p=(0,n.useContext)(l.Q).assignId,w=(0,o.L)("heading");return void 0!==p&&f&&p(w),n.createElement(n.Fragment,null,f?n.createElement("span",{id:w},t):t,(r||c||u)&&n.createElement("div",{ref:_,className:(0,i.Z)(a.Z.tools,"default"===h&&a.Z["tools-small"])},r&&n.createElement("div",{className:a.Z["tools-filtering"],onClickCapture:()=>null==d?void 0:d("filter"),onKeyDownCapture:()=>null==d?void 0:d("filter")},r),n.createElement("div",{className:a.Z["tools-align-right"]},c&&n.createElement("div",{className:a.Z["tools-pagination"],onClickCapture:()=>null==d?void 0:d("pagination")},c),u&&n.createElement("div",{className:a.Z["tools-preferences"],onClickCapture:()=>null==d?void 0:d("preferences")},u))))}},4705:function(e,t,r){"use strict";r.d(t,{LF:function(){return n},PT:function(){return u},_0:function(){return s},ai:function(){return l},hS:function(){return c},mU:function(){return a},sA:function(){return d},uu:function(){return o}});var i=r(9024);let n=(e,t)=>"function"==typeof e?e(t):t[e],s=(e,t,r)=>e?n(e,t):r,a=(e,t)=>e?n(e,t):t,l=(e,t)=>e.id||t,o=e=>e&&"container"!==e?"borderless"===e?"embedded":e:"default";function c(e,t){let r=e.filter(e=>e.sortingComparator===t)[0];r||(0,i.O4)("Table","Currently active sorting comparator was not found in any columns. Make sure to provide the same comparator function instance on each render.")}function u(e){let{columnDisplay:t,visibleColumns:r,columnDefinitions:i}=e;return t?function(e){let{columnDisplay:t,columnDefinitions:r}=e,i=r.reduce((e,t)=>void 0===t.id?e:Object.assign(Object.assign({},e),{[t.id]:t}),{});return t.filter(e=>e.visible).map(e=>i[e.id]).filter(Boolean)}({columnDisplay:t,columnDefinitions:i}):r?function(e){let{visibleColumns:t,columnDefinitions:r}=e,i=new Set(t);return r.filter(e=>{let{id:t}=e;return void 0!==t&&i.has(t)})}({visibleColumns:r,columnDefinitions:i}):i}function d(e,t){return{[e["sticky-cell"]]:!!t,[e["sticky-cell-pad-inline-start"]]:!!(null==t?void 0:t.padInlineStart),[e["sticky-cell-last-inline-start"]]:!!(null==t?void 0:t.lastInsetInlineStart),[e["sticky-cell-last-inline-end"]]:!!(null==t?void 0:t.lastInsetInlineEnd)}}},9329:function(){},9383:function(){},5300:function(){},553:function(){},1527:function(){},7085:function(){},1869:function(){},777:function(e,t,r){"use strict";let i;r.d(t,{a:function(){return C}});var n=r(4139),s=r(7037),a=r(6474),l=r(7506),o=r(6888),c=class extends l.l{constructor(e,t){super(),this.options=t,this.#e=e,this.#t=null,this.bindMethods(),this.setOptions(t)}#e;#r=void 0;#i=void 0;#n=void 0;#s;#a;#t;#l;#o;#c;#u;#d;#h;#_=new Set;bindMethods(){this.refetch=this.refetch.bind(this)}onSubscribe(){1===this.listeners.size&&(this.#r.addObserver(this),u(this.#r,this.options)?this.#f():this.updateResult(),this.#p())}onUnsubscribe(){this.hasListeners()||this.destroy()}shouldFetchOnReconnect(){return d(this.#r,this.options,this.options.refetchOnReconnect)}shouldFetchOnWindowFocus(){return d(this.#r,this.options,this.options.refetchOnWindowFocus)}destroy(){this.listeners=new Set,this.#w(),this.#g(),this.#r.removeObserver(this)}setOptions(e,t){let r=this.options,i=this.#r;if(this.options=this.#e.defaultQueryOptions(e),void 0!==this.options.enabled&&"boolean"!=typeof this.options.enabled&&"function"!=typeof this.options.enabled&&"boolean"!=typeof(0,n.Nc)(this.options.enabled,this.#r))throw Error("Expected enabled to be a boolean or a callback that returns a boolean");this.#m(),this.#r.setOptions(this.options),r._defaulted&&!(0,n.VS)(this.options,r)&&this.#e.getQueryCache().notify({type:"observerOptionsUpdated",query:this.#r,observer:this});let s=this.hasListeners();s&&h(this.#r,i,this.options,r)&&this.#f(),this.updateResult(t),s&&(this.#r!==i||(0,n.Nc)(this.options.enabled,this.#r)!==(0,n.Nc)(r.enabled,this.#r)||(0,n.KC)(this.options.staleTime,this.#r)!==(0,n.KC)(r.staleTime,this.#r))&&this.#y();let a=this.#v();s&&(this.#r!==i||(0,n.Nc)(this.options.enabled,this.#r)!==(0,n.Nc)(r.enabled,this.#r)||a!==this.#h)&&this.#b(a)}getOptimisticResult(e){let t=this.#e.getQueryCache().build(this.#e,e),r=this.createResult(t,e);return(0,n.VS)(this.getCurrentResult(),r)||(this.#n=r,this.#a=this.options,this.#s=this.#r.state),r}getCurrentResult(){return this.#n}trackResult(e,t){let r={};return Object.keys(e).forEach(i=>{Object.defineProperty(r,i,{configurable:!1,enumerable:!0,get:()=>(this.trackProp(i),t?.(i),e[i])})}),r}trackProp(e){this.#_.add(e)}getCurrentQuery(){return this.#r}refetch({...e}={}){return this.fetch({...e})}fetchOptimistic(e){let t=this.#e.defaultQueryOptions(e),r=this.#e.getQueryCache().build(this.#e,t);return r.isFetchingOptimistic=!0,r.fetch().then(()=>this.createResult(r,t))}fetch(e){return this.#f({...e,cancelRefetch:e.cancelRefetch??!0}).then(()=>(this.updateResult(),this.#n))}#f(e){this.#m();let t=this.#r.fetch(this.options,e);return e?.throwOnError||(t=t.catch(n.ZT)),t}#y(){this.#w();let r=(0,n.KC)(this.options.staleTime,this.#r);if(n.sk||this.#n.isStale||!(0,n.PN)(r))return;let i=(0,n.Kp)(this.#n.dataUpdatedAt,r);this.#u=setTimeout(()=>{this.#n.isStale||this.updateResult()},i+1)}#v(){return("function"==typeof this.options.refetchInterval?this.options.refetchInterval(this.#r):this.options.refetchInterval)??!1}#b(c){this.#g(),this.#h=c,!n.sk&&!1!==(0,n.Nc)(this.options.enabled,this.#r)&&(0,n.PN)(this.#h)&&0!==this.#h&&(this.#d=setInterval(()=>{(this.options.refetchIntervalInBackground||a.j.isFocused())&&this.#f()},this.#h))}#p(){this.#y(),this.#b(this.#v())}#w(){this.#u&&(clearTimeout(this.#u),this.#u=void 0)}#g(){this.#d&&(clearInterval(this.#d),this.#d=void 0)}createResult(e,t){let r;let i=this.#r,s=this.options,a=this.#n,l=this.#s,c=this.#a,d=e!==i?e.state:this.#i,{state:f}=e,p={...f},w=!1;if(t._optimisticResults){let g=this.hasListeners(),m=!g&&u(e,t),y=g&&h(e,i,t,s);(m||y)&&(p={...p,...(0,o.z)(f.data,e.options)}),"isRestoring"===t._optimisticResults&&(p.fetchStatus="idle")}let{error:v,errorUpdatedAt:b,status:k}=p;if(t.select&&void 0!==p.data){if(a&&p.data===l?.data&&t.select===this.#l)r=this.#o;else try{this.#l=t.select,r=t.select(p.data),r=(0,n.oE)(a?.data,r,t),this.#o=r,this.#t=null}catch(x){this.#t=x}}else r=p.data;if(void 0!==t.placeholderData&&void 0===r&&"pending"===k){let j;if(a?.isPlaceholderData&&t.placeholderData===c?.placeholderData)j=a.data;else if(j="function"==typeof t.placeholderData?t.placeholderData(this.#c?.state.data,this.#c):t.placeholderData,t.select&&void 0!==j)try{j=t.select(j),this.#t=null}catch(R){this.#t=R}void 0!==j&&(k="success",r=(0,n.oE)(a?.data,j,t),w=!0)}this.#t&&(v=this.#t,r=this.#o,b=Date.now(),k="error");let E="fetching"===p.fetchStatus,C="pending"===k,O="error"===k,I=C&&E,Z=void 0!==r,S={status:k,fetchStatus:p.fetchStatus,isPending:C,isSuccess:"success"===k,isError:O,isInitialLoading:I,isLoading:I,data:r,dataUpdatedAt:p.dataUpdatedAt,error:v,errorUpdatedAt:b,failureCount:p.fetchFailureCount,failureReason:p.fetchFailureReason,errorUpdateCount:p.errorUpdateCount,isFetched:p.dataUpdateCount>0||p.errorUpdateCount>0,isFetchedAfterMount:p.dataUpdateCount>d.dataUpdateCount||p.errorUpdateCount>d.errorUpdateCount,isFetching:E,isRefetching:E&&!C,isLoadingError:O&&!Z,isPaused:"paused"===p.fetchStatus,isPlaceholderData:w,isRefetchError:O&&Z,isStale:_(e,t),refetch:this.refetch};return S}updateResult(e){let t=this.#n,r=this.createResult(this.#r,this.options);if(this.#s=this.#r.state,this.#a=this.options,void 0!==this.#s.data&&(this.#c=this.#r),(0,n.VS)(r,t))return;this.#n=r;let i={};e?.listeners!==!1&&(()=>{if(!t)return!0;let{notifyOnChangeProps:e}=this.options,r="function"==typeof e?e():e;if("all"===r||!r&&!this.#_.size)return!0;let i=new Set(r??this.#_);return this.options.throwOnError&&i.add("error"),Object.keys(this.#n).some(e=>{let r=this.#n[e]!==t[e];return r&&i.has(e)})})()&&(i.listeners=!0),this.#k({...i,...e})}#m(){let f=this.#e.getQueryCache().build(this.#e,this.options);if(f===this.#r)return;let p=this.#r;this.#r=f,this.#i=f.state,this.hasListeners()&&(p?.removeObserver(this),f.addObserver(this))}onQueryUpdate(){this.updateResult(),this.hasListeners()&&this.#p()}#k(w){s.V.batch(()=>{w.listeners&&this.listeners.forEach(e=>{e(this.#n)}),this.#e.getQueryCache().notify({query:this.#r,type:"observerResultsUpdated"})})}};function u(e,t){return!1!==(0,n.Nc)(t.enabled,e)&&void 0===e.state.data&&!("error"===e.state.status&&!1===t.retryOnMount)||void 0!==e.state.data&&d(e,t,t.refetchOnMount)}function d(e,t,r){if(!1!==(0,n.Nc)(t.enabled,e)){let i="function"==typeof r?r(e):r;return"always"===i||!1!==i&&_(e,t)}return!1}function h(e,t,r,i){return(e!==t||!1===(0,n.Nc)(i.enabled,e))&&(!r.suspense||"error"!==e.state.status)&&_(e,r)}function _(e,t){return!1!==(0,n.Nc)(t.enabled,e)&&e.isStaleByTime((0,n.KC)(t.staleTime,e))}var f=r(7294);r(5893);var p=f.createContext((i=!1,{clearReset:()=>{i=!1},reset:()=>{i=!0},isReset:()=>i})),w=()=>f.useContext(p),g=r(202),m=f.createContext(!1),y=()=>f.useContext(m);m.Provider;var v=r(6290),b=(e,t)=>{(e.suspense||e.throwOnError)&&!t.isReset()&&(e.retryOnMount=!1)},k=e=>{f.useEffect(()=>{e.clearReset()},[e])},x=({result:e,errorResetBoundary:t,throwOnError:r,query:i})=>e.isError&&!t.isReset()&&!e.isFetching&&i&&(0,v.L)(r,[e.error,i]),j=e=>{e.suspense&&"number"!=typeof e.staleTime&&(e.staleTime=1e3)},R=(e,t)=>e?.suspense&&t.isPending,E=(e,t,r)=>t.fetchOptimistic(e).catch(()=>{r.clearReset()});function C(e,t){return function(e,t,r){let i=(0,g.NL)(r),n=y(),a=w(),l=i.defaultQueryOptions(e);i.getDefaultOptions().queries?._experimental_beforeQuery?.(l),l._optimisticResults=n?"isRestoring":"optimistic",j(l),b(l,a),k(a);let[o]=f.useState(()=>new t(i,l)),c=o.getOptimisticResult(l);if(f.useSyncExternalStore(f.useCallback(e=>{let t=n?()=>void 0:o.subscribe(s.V.batchCalls(e));return o.updateResult(),t},[o,n]),()=>o.getCurrentResult(),()=>o.getCurrentResult()),f.useEffect(()=>{o.setOptions(l,{listeners:!1})},[l,o]),R(l,c))throw E(l,o,a);if(x({result:c,errorResetBoundary:a,throwOnError:l.throwOnError,query:i.getQueryCache().get(l.queryHash)}))throw c.error;return i.getDefaultOptions().queries?._experimental_afterQuery?.(l,c),l.notifyOnChangeProps?c:o.trackResult(c)}(e,c,t)}},6290:function(e,t,r){"use strict";function i(e,t){return"function"==typeof e?e(...t):!!e}function n(){}r.d(t,{L:function(){return i},Z:function(){return n}})}}]);