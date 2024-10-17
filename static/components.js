// create a web component with name 'cmp-metadata'
// and define its template and style, without using framework

const ABC = ' ABCDEFGHIJKLMNOPQRSTUVWXYZ'
const calcCombination = n => (ABC[Math.floor(n / ABC.length)] + ABC[n % ABC.length]).trim()
class Metadata extends HTMLElement {
    // convert the classe to css style and add it to the shadow root
    constructor() {
        super();
        this.root = this.attachShadow({ mode: 'open' });
        this.root.innerHTML = `
        <style>
            .metadata {
            display: flex;
            flex-direction: row;
            gap: 10px;
            }
            h3 {
            font-size: 15px;
            font-weight: 300;
            line-height: 24.2px;
            color: var(--font-primary-color);
            }
            strong {
            font-weight: 700;
            color: var(--font-tertiary-color);
            }
        </style>
        <div class="metadata">
            <h3> Tiempo Limite: <strong> <slot name="time-limit"></slot> s </strong> </h3>
            <h3> Limite de memoria: <strong> <slot name="memory-limit"></slot> MB </strong> </h3>
        </div>
        `;
        // htmx.process(this.root) // Tell HTMX about this component's shadow DOM
    }
}

class Title extends HTMLElement {
    constructor() {
        super();
        this.root = this.attachShadow({ mode: 'open' });
        this.root.innerHTML = `
        <style>
            h2 {
            font-weight: 900;
            font-size: 30px;
            line-height: 36.31px;
            color: var(--font-tertiary-color);
            }
            .container {
            display: flex;
            flex-direction: column;
            width: 100%;
            }
        </style>
        <div class="container">
            <h2> <slot name="title"></slot> </h2>
        </div>
        `;
    }
}

class Subtitle extends HTMLElement {
    constructor() {
        super();
        this.root = this.attachShadow({ mode: 'open' });
        this.root.innerHTML = `
        <style>
        h2 {
            font-weight: 700;
            font-size: 25px;
            line-height: 30.26px;
            margin: 0;
            color: var(--font-tertiary-color);
        }
        .container {
            display: flex;
            flex-direction: column;
            width: 100%;
        }
        </style>
        <div class="container">
            <h2> ${this.getAttribute('text')} </h2>
        </div>
        `;
    }
}
class Problem extends HTMLElement {
    constructor() {
        super();
        this.root = this.attachShadow({ mode: 'open' });
        this.root.innerHTML = `
        <style>
            h2 {
                font-weight: 700;
                font-size: 25px;
                line-height: 30.26px;
                margin: 0;
                color: var(--font-tertiary-color);
            }
            .container {
                margin-top: 10px;
                display: flex;
                flex-direction: column;
                gap: 10px;
            }
            .examples {
                display: flex;
                flex-direction: column;
                gap: 10px;
            }
        </style>
        <div class="container">
            <div slot="content">
                <slot name="problem"></slot>
            </div>
            <cmp-problem-block title="Input">
                <div slot="content">
                    <slot name="input"></slot>
                </div>
            </cmp-problem-block>
            <cmp-problem-block title="Output">
                <div slot="content">
                    <slot name="output"></slot>
                </div>
            </cmp-problem-block>
            <cmp-problem-block title="Examples">
                <div slot="content">
                    <div class="examples">
                        <slot name="examples"></slot>
                    </div>
                </div>
            </cmp-problem-block>
        </div>
        `;
    }
}

class ProblemBlock extends HTMLElement {
    constructor() {
        super();
        this.root = this.attachShadow({ mode: 'open' });
        this.root.innerHTML = `
        <style>
            .container {
                display: flex;
                flex-direction: column;
                gap: 5px;
            }
            hr {
                border: 0.3px solid var(--border-color);
                width: 100%;
            }
        </style>
        <div class="container">
            <cmp-subtitle text="${this.getAttribute('title')}"></cmp-subtitle>
            <hr/>
            <slot name="content"></slot>
        </div>
        `;
    }
}

class ProblemExample extends HTMLElement {
    constructor() {
        super();
        this.root = this.attachShadow({ mode: 'open' });
        this.root.innerHTML = `
        <style>
            .container {
                display: flex;
                flex-direction: column;
                gap: 5px;
                border: 1px solid var(--border-color);
                border-radius: var(--border-radius);
                padding: 10px;
            }
            .block {
                display: flex;
                flex-direction: column;
                gap: 1px;
            }
            h3, pre {
                margin: 0;
            }
        </style>
        <div class="container">
            <div class="block">
                <h3>Input</h3>
                <pre><code><slot name="input"></slot> </code></pre>
            </div>
            <div class="block">
                <h3>Output</h3>
                <pre><code><slot name="output"></slot> </code></pre>
            </div>
        </div>
        `;
    }
}


class Tabs extends HTMLElement {
    constructor() {
        super();
        this.root = this.attachShadow({ mode: 'open' });
        let tabs = []
        document.querySelectorAll('cmp-tab').forEach(tab => {
            // tab.addEventListener('click', () => {
            //     document.querySelectorAll('cmp-tab').forEach((tab) => {
            //         console.log(tab)
            //         tab.setAttribute("active", false);
            //     });
            //     tab.setAttribute("active", true);
            // });
            tabs.push({ name: tab.getAttribute('name'), title: tab.getAttribute('title'), active: tab.getAttribute('active') === 'true' })
        })

        this.root.innerHTML = `
            <style>
                ul {
                    all: unset;
                    display: flex;
                    width: 100%;
                    gap: 40px;
                    border-bottom: 1px solid var(--border-color);
                    background: ${this.getAttribute('background') || 'transparent'};
                    ${
                        this.getAttribute('is-contest') === 'true' ?  'padding-left: 20px' : ''
                    }
                }
                li {
                    all: unset;
                    font-size: 16px;
                    font-weight: 400;
                    color: var(--font-primary-color);
                    cursor: pointer;
                    position: relative;
                }
                li[active="true"] {
                    color: var(--blue-primary);
                }
                li[active="true"]::after {
                    content: '';
                    width: 100%;
                    bottom: -1px;
                    left: 0;
                    border-bottom: 1px solid var(--blue-primary);
                    position: absolute;
                }
                .container {
                    display: flex;
                    flex-direction: column;
                }
            </style>
            <div id="container" class="container">
                <slot id="tab-slot"></slot>
            </div>
        `
    }
    connectedCallback() {
        const container = this.shadowRoot.querySelector('#container')
        const ul = document.createElement('ul')
        const slots = this.shadowRoot.querySelector('#tab-slot').assignedElements({ flatten: true })
        let tabs = []
        slots.forEach(tab => {
            // tab.addEventListener('click', () => {
            //     document.querySelectorAll('cmp-tab').forEach((tab) => {
            //         console.log(tab)
            //         tab.setAttribute("active", false);
            //     });
            //     tab.setAttribute("active", true);
            // });
            tabs.push({ name: tab.getAttribute('name'), title: tab.getAttribute('title'), active: tab.getAttribute('active') === 'true' })
        })
        ul.innerHTML = `
            ${tabs.map(tab => `
                <li
                    active=${tab.active ? '"true"' : '"false"'}
                    class="tab" name="${tab.name}">
                    ${tab.title}
                </li>
            `).join(' ')}
        `
        container.prepend(ul)
        this.shadowRoot.querySelectorAll('li.tab').forEach(li => {
            li.addEventListener('click', () => {
                if (li.getAttribute('name') === 'submissions') {
                    document.dispatchEvent(new CustomEvent("submissionClicked", {
                        bubbles: true,
                        cancelable: false,
                        composed: true
                    }))
                }
                // this.shadowRoot.querySelectorAll('cmp-tab').forEach((tab) => {
                this.shadowRoot.querySelector('#tab-slot').assignedElements({ flatten: true }).forEach((tab) => {
                    if (tab.getAttribute('name') !== li.getAttribute('name')) {
                        tab.setAttribute("active", false)
                    } else {
                        tab.setAttribute("active", true)
                    }
                });

                this.shadowRoot.querySelectorAll('li.tab').forEach((tab) => {
                    if (tab.getAttribute('name') !== li.getAttribute('name')) {
                        tab.setAttribute("active", false);
                    } else {
                        tab.setAttribute("active", true)
                    }
                })

                if(this.getAttribute('is-contest') === 'true') {
                    // set has of the page to the current tab
                    window.location.hash = li.getAttribute('name')
                }
            })
        })
    }
    static observedAttributes = ["active-tab"];

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === 'active-tab') {
            this.shadowRoot.querySelector('#tab-slot').assignedElements({ flatten: true }).forEach((tab) => {
                if (tab.getAttribute('name') !== newValue) {
                    tab.setAttribute("active", false)
                } else {
                    tab.setAttribute("active", true)
                }
            });

            this.shadowRoot.querySelectorAll('li.tab').forEach((tab) => {
                if (tab.getAttribute('name') !== newValue) {
                    tab.setAttribute("active", false);
                } else {
                    tab.setAttribute("active", true)
                }
            })
        }
    }
}

class Tab extends HTMLElement {
    constructor() {
        super();
        this.root = this.attachShadow({ mode: 'open' });
        this.root.innerHTML = `
        <style>
            .container {
                display: flex;
                flex-direction: column;
                display: ${this.hasAttribute('active') ? 'block' : 'none'}
                background-color: red;
            }
        </style>
        <div class="tab container">
            <slot></slot>
        </div>
        `;
    }
    static observedAttributes = ["active"];

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === 'active') {
            if (newValue === 'true') {
                this.shadowRoot.querySelector('.container').style.display = 'block'
            } else {
                this.shadowRoot.querySelector('.container').style.display = 'none'
            }
        }
    }
}
class Hr extends HTMLElement {
    constructor() {
        super();
        this.root = this.attachShadow({ mode: 'open' });
        this.root.innerHTML = `
        <style>
            hr {
                border: 0.3px solid var(--border-color);
                width: 100%;
            }
        </style>
        <hr/>
        `;
    }
}

class Submit extends HTMLElement {
    constructor() {
        super();
        this.value = this.attachShadow({ mode: 'open' });

        let problems = []
        for (let idx = 1; idx <= parseInt(this.getAttribute('contest-range')); idx++) {
            problems.push(calcCombination(idx))
        }
        this.value.innerHTML = `
        <style>
            .container {
                display: flex;
                flex-direction: column;
                gap: 5px;
            }
            select {
                all:unset;
                width: 10vw;
                height: fit-content;
                padding: 5px;
                border: 1px solid var(--border-color);
                border-radius: 5px;
                font-size: 15px;
                color: white;
            }
            h3 {
                font-size: 15px;
                font-weight: 400;
                line-height: 24.2px;
                color: white;
            }
            .title {
                display: flex;
                gap: 10px;
                align-items: center;
                width: 100%;
            }
            .input {
                border-radius: var(--border-radius);
                border: var(--border-style);
                background: var(--secondary-color);
                color: var(--font-primary-color);
                width: 100%;
                padding: 5px;
            }
            .top-bar {
                display: flex;
                justify-content: space-between;
                align-items: center;
            }
        </style>
        <div class="container">
            <form
                hx-post="/api/submit"
                enctype="multipart/form-data"
                hx-trigger="postSubmitForm from:document"
                hx-
            >
                <div class="top-bar">
                    <div class="title">
                        <h3>Selecciona el lenguaje de programación</h3>
                        <select name="language">
                            <option value="cpp17">C++ 17</option>
                            <option value="python3">Python 3</option>
                            <option value="javascript">Node.js</option>
                        </select>
                    </div>
                    
                    <cmp-button
                        event="postSubmitForm"
                    >Enviar</cmp-button>
                </div>
                    ${this.getAttribute('contest') === "true" ? `
                        <div class="title">
                        <h3>Selecciona el problema</h3>
                        <select name="problem">
                            ${problems.map(problem => `
                                <option value="${problem}">${problem}</option>
                            `).join(' ')}
                        </select>
                    </div>
                    `: ''}
                <div class="title">
                    <h3>Selecciona archivo</h3>
                    <input type="file" id="file" name="code" accept=".cpp,.py,.js, .java, .c">
                </div>
                <!-- <cmp-hr></cmp-hr> -->
                <!-- <h3>Tambien puedes  tu aqui</h3> -->
                <!-- <textarea class="input" name="code" id="code" cols="30" rows="20"></textarea> -->
                <input type="hidden" id="file" name="contest_id" value="${this.getAttribute('contest-id')}">
                <input type="hidden" id="file" name="problem_id" value="${this.getAttribute('problem-id')}">
            </form>
        </div>
        `;
    }

    connectedCallback() {
        htmx.process(this.shadowRoot)
    }
}

class SubmissionsTable extends HTMLElement {
    constructor() {
        super();
        this.root = this.attachShadow({ mode: 'open' });
        this.root.innerHTML = `
        <style>
            table {
                width: 100%;
                   border-collapse: collapse;
            }
            th,
            td {
                color: var(--font-primary-color);
                //border-radius: var(--border-radius);
                border-bottom: 1px solid var(--border-color);
                border-left: 1px solid var(--border-color);
                text-align: center;
                padding: 5px;
                   border-collapse: collapse;
            }

            th:last-of-type,
            td:last-of-type {
                border-right: 1px solid var(--border-color);
            }

            th {
                border-top: 1px solid var(--border-color);
                color: white;
            }

            th:first-of-type {
                border-top-left-radius: 10px;
            }
            th:last-of-type {
                border-top-right-radius: 10px;
            }
            table tr:last-of-type td:first-of-type {
                border-bottom-left-radius: 10px;
            }
            table tr:last-of-type td:last-of-type {
                border-bottom-right-radius: 10px;
            }
            table td {
                padding: 10px;
            }

        </style>
        <table>
            <tr>
                <th>Submission id</th>
                <th>Lenguaje</th>
                <th>Estado</th>
                <th>Fecha</th>
            </tr>
            <tr>
                <td>1</td>
                <td>C++</td>
                <td>AC</td>
                <td>2021-09-20 12:00:00</td>
            </tr>
            <tr>
                <td>2</td>
                <td>C++</td>
                <td>AC</td>
                <td>2021-09-20 12:00:00</td>
            </tr>
            <tr>
                <td>3</td>
                <td>C++</td>
                <td>AC</td>
                <td>2021-09-20 12:00:00</td>
            </tr>
        </table>
            <slot id="rows"></slot>
        `;
    }
    connectedCallback() {
        let objects = this.shadowRoot.querySelector('#rows').assignedElements({ flatten: true });
        console.log(objects)
    }
}

class Button extends HTMLElement {
    constructor() {
        super();
        this.root = this.attachShadow({ mode: 'open' });
        this.root.innerHTML = `
        <style>
            button {
                background: var(--blue-primary);
                border: 1px solid var(--blue-secondary);
                border-radius: var(--border-radius);
                width: 126px;
                padding: 0 10px 0 10px;
                height: 29px;
                cursor: pointer;
            }
            button:active:hover {
                background: var(--blue-primary-faded);
            }
        </style>
        <button id="button" >
            <slot></slot>
        </button>
        `;
    }
    connectedCallback() {
        this.shadowRoot.querySelector('button').addEventListener('click', () => {
            document.dispatchEvent(new CustomEvent(this.getAttribute('event'), {
                bubbles: true,
                cancelable: false,
                composed: true
            }))
        })
    }
}

class ProgressBar extends HTMLElement {
    constructor() {
        super();
        this.root = this.attachShadow({ mode: 'open' });
        this.root.innerHTML = `
        <style>
            progress {
                width: 100%;
     }
        </style>
        <div>
            <progress class="progress progress-primary w-56" value="0" max="100"></progress>
        </div>
        `;
    }
    connectedCallback() {
        this.shadowRoot.querySelector('.progress-bar').style.width = this.getAttribute('value') + '%'
    }
}

customElements.define('cmp-metadata', Metadata);
customElements.define('cmp-title', Title);
customElements.define('cmp-subtitle', Subtitle);
customElements.define('cmp-problem', Problem);
customElements.define('cmp-problem-block', ProblemBlock);
customElements.define('cmp-tabs', Tabs);
customElements.define('cmp-tab', Tab);
customElements.define('cmp-submit', Submit);
customElements.define('cmp-hr', Hr);
customElements.define('cmp-button', Button);
customElements.define('cmp-submissions', SubmissionsTable);
customElements.define('cmp-problem-example', ProblemExample);