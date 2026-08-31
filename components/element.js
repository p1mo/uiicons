class UiIconElement extends HTMLElement {  

    ui_name   = "[UI_NAME]";
    ui_size   = "[UI_SIZE]";
    ui_color  = "[UI_COLOR]";
    ui_parser = new DOMParser();

    constructor() {
        super();
    }

    connectedCallback() {
        this.ui_name = this.getAttribute('name') || this.ui_name;
        this.ui_size = this.getAttribute('size') || this.ui_size;
        this.ui_color = this.getAttribute('color') || this.ui_color;
        this.icon(this.ui_name);
    }

    async icon(name) {        
        const icon = await this.fetchSVG(name);
        const svg = this.ui_parser.parseFromString(icon.replace(/^"|"$|\\/g, ""), "image/svg+xml");
        svg.documentElement.style.width = "100%";
        svg.documentElement.style.height = "100%";
        svg.documentElement.setAttribute("width",  "100%");
        svg.documentElement.setAttribute("height", "100%");
        if (this.ui_color === "fill") {
            svg.documentElement.style.fill   = "currentColor";
            svg.documentElement.style.stroke = "transparent";
        } else if (this.ui_color === "stroke") {
            svg.documentElement.style.fill   = "transparent";
            svg.documentElement.style.stroke = "currentColor";
        }
        this.innerHTML = svg.documentElement.outerHTML;
    }

    async fetchSVG(name) {
        const req = await fetch(`/uiicons/${name}.svg`);
        return await req.text()
    }

}
  
customElements.define("[COMPONENT_NAME]", UiIconElement);