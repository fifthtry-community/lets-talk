class DateTimePicker extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: "open" });
        this._selectedDate = new Date();
        this._outputFormat = "iso"; // Default format: ISO string
        this._onChangeCallback = null;
        this.render();
    }

    static get observedAttributes() {
        return ["value", "min-date", "max-date", "format"];
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (name === "value" && newValue) {
            this._selectedDate = new Date(newValue);
            this.updateInputs();
        }
        if (name === "format" && newValue) {
            this._outputFormat = newValue;
        }
    }

    connectedCallback() {
        this.shadowRoot
            .querySelector(".date-input")
            .addEventListener("change", (e) => {
                const newDate = new Date(
                    e.target.value + "T" + this.getTimeString()
                );
                this.setDateTime(newDate);
            });

        this.shadowRoot
            .querySelector(".time-input")
            .addEventListener("change", (e) => {
                const date = this.getDateString();
                const newDate = new Date(date + "T" + e.target.value);
                this.setDateTime(newDate);
            });
    }

    getDateString() {
        return this._selectedDate.toISOString().split("T")[0];
    }

    getTimeString() {
        return this._selectedDate.toTimeString().substring(0, 5);
    }

    // Convert the date to desired string format
    formatDateToString(date) {
        switch (this._outputFormat) {
            case "iso":
                return date.toISOString();
            case "short":
                return `${this.getDateString()} ${this.getTimeString()}`;
            case "locale":
                return date.toLocaleString();
            case "date-only":
                return this.getDateString();
            case "time-only":
                return this.getTimeString();
            default:
                return date.toISOString();
        }
    }

    setDateTime(date) {
        this._selectedDate = date;
        this.updateInputs();

        const formattedDate = this.formatDateToString(this._selectedDate);

        if (this._onChangeCallback) {
            this._onChangeCallback(formattedDate);
        }

        this.dispatchEvent(
            new CustomEvent("change", {
                detail: {
                    value: formattedDate,
                    rawDate: this._selectedDate,
                },
                bubbles: true,
            })
        );
    }

    updateInputs() {
        const dateInput = this.shadowRoot.querySelector(".date-input");
        const timeInput = this.shadowRoot.querySelector(".time-input");

        if (dateInput && timeInput) {
            dateInput.value = this.getDateString();
            timeInput.value = this.getTimeString();
        }
    }

    render() {
        this.shadowRoot.innerHTML = `
        <style>
          :host {
            display: inline-block;
            font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
          }
          .container {
            display: flex;
            flex-direction: column;
            gap: 10px;
            padding: 10px;
            border-radius: 8px;
            background-color: #f5f5f5;
            box-shadow: 0 2px 5px rgba(0,0,0,0.1);
          }
          input {
            padding: 8px;
            border: 1px solid #ccc;
            border-radius: 4px;
            font-size: 16px;
          }
          label {
            display: block;
            margin-bottom: 4px;
            font-size: 14px;
            font-weight: 500;
            color: #333;
          }
          .input-group {
            margin-bottom: 8px;
          }
        </style>
        <div class="container">
          <div class="input-group">
            <label>Date</label>
            <input type="date" class="date-input" value="${this.getDateString()}">
          </div>
          <div class="input-group">
            <label>Time</label>
            <input type="time" class="time-input" value="${this.getTimeString()}">
          </div>
        </div>
      `;
    }

    get value() {
        return this.formatDateToString(this._selectedDate);
    }

    set value(newValue) {
        if (newValue instanceof Date) {
            this._selectedDate = new Date(newValue);
        } else {
            this._selectedDate = new Date(newValue);
        }
        this.updateInputs();
    }

    // Allow setting the output format
    set format(formatType) {
        this._outputFormat = formatType;
    }

    get format() {
        return this._outputFormat;
    }

    onChange(callback) {
        this._onChangeCallback = callback;
    }
}

// Register the web component
customElements.define("date-time-picker", DateTimePicker);

// Example usage:
// <date-time-picker value="2025-02-28T15:30:00" format="short"></date-time-picker>
