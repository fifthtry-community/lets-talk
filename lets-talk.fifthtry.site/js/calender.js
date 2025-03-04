(function () {
    class DateTimePicker extends HTMLElement {
        constructor() {
            super();
            this.attachShadow({ mode: "open" });
            this._selectedStartDate = new Date();
            this._selectedEndDate = new Date();
            this._onChangeCallback = null;
            this.render();
        }

        static get observedAttributes() {
            return [
                "start_value",
                "end_value",
                "min-date",
                "max-date",
                "format",
            ];
        }

        attributeChangedCallback(name, oldValue, newValue) {
            if (name === "start_value" && newValue) {
                this._selectedStartDate = new Date(newValue);
                this.updateInputs();
            }
            if (name === "end_value" && newValue) {
                this._selectedEndDate = new Date(newValue);
                this.updateInputs();
            }
        }

        connectedCallback() {
            // Start date input
            this.shadowRoot
                .querySelectorAll(".date-input")[0]
                .addEventListener("change", (e) => {
                    // Get current date parts
                    const dateParts = e.target.value.split("-");
                    const year = parseInt(dateParts[0]);
                    const month = parseInt(dateParts[1]) - 1; // Months are 0-indexed in JS
                    const day = parseInt(dateParts[2]);

                    // Get current time parts
                    const timeParts = this.getStartTimeString().split(":");
                    const hour = parseInt(timeParts[0]);
                    const minute = parseInt(timeParts[1]);

                    // Create new date preserving local time
                    const newDate = new Date();
                    newDate.setFullYear(year, month, day);
                    newDate.setHours(hour, minute, 0, 0);

                    this.setStartDateTime(newDate);
                });

            // End date input
            this.shadowRoot
                .querySelectorAll(".date-input")[1]
                .addEventListener("change", (e) => {
                    // Get current date parts
                    const dateParts = e.target.value.split("-");
                    const year = parseInt(dateParts[0]);
                    const month = parseInt(dateParts[1]) - 1; // Months are 0-indexed in JS
                    const day = parseInt(dateParts[2]);

                    // Get current time parts
                    const timeParts = this.getEndTimeString().split(":");
                    const hour = parseInt(timeParts[0]);
                    const minute = parseInt(timeParts[1]);

                    // Create new date preserving local time
                    const newDate = new Date();
                    newDate.setFullYear(year, month, day);
                    newDate.setHours(hour, minute, 0, 0);

                    this.setEndDateTime(newDate);
                });

            // Start time input
            this.shadowRoot
                .querySelectorAll(".time-input")[0]
                .addEventListener("change", (e) => {
                    // Get current date parts
                    const current = this._selectedStartDate;
                    const year = current.getFullYear();
                    const month = current.getMonth();
                    const day = current.getDate();

                    // Get new time parts
                    const timeParts = e.target.value.split(":");
                    const hour = parseInt(timeParts[0]);
                    const minute = parseInt(timeParts[1]);

                    // Create new date preserving local time
                    const newDate = new Date();
                    newDate.setFullYear(year, month, day);
                    newDate.setHours(hour, minute, 0, 0);

                    this.setStartDateTime(newDate);
                });

            // End time input
            this.shadowRoot
                .querySelector(".end-time-input")
                .addEventListener("change", (e) => {
                    // Get current date parts
                    const current = this._selectedEndDate;
                    const year = current.getFullYear();
                    const month = current.getMonth();
                    const day = current.getDate();

                    // Get new time parts
                    const timeParts = e.target.value.split(":");
                    const hour = parseInt(timeParts[0]);
                    const minute = parseInt(timeParts[1]);

                    // Create new date preserving local time
                    const newDate = new Date();
                    newDate.setFullYear(year, month, day);
                    newDate.setHours(hour, minute, 0, 0);

                    this.setEndDateTime(newDate);
                });
        }

        getStartDateString() {
            const year = this._selectedStartDate.getFullYear();
            const month = String(
                this._selectedStartDate.getMonth() + 1
            ).padStart(2, "0");
            const day = String(this._selectedStartDate.getDate()).padStart(
                2,
                "0"
            );
            return `${year}-${month}-${day}`;
        }

        getEndDateString() {
            const year = this._selectedEndDate.getFullYear();
            const month = String(this._selectedEndDate.getMonth() + 1).padStart(
                2,
                "0"
            );
            const day = String(this._selectedEndDate.getDate()).padStart(
                2,
                "0"
            );
            return `${year}-${month}-${day}`;
        }

        getStartTimeString() {
            const hours = String(this._selectedStartDate.getHours()).padStart(
                2,
                "0"
            );
            const minutes = String(
                this._selectedStartDate.getMinutes()
            ).padStart(2, "0");
            return `${hours}:${minutes}`;
        }

        getEndTimeString() {
            const hours = String(this._selectedEndDate.getHours()).padStart(
                2,
                "0"
            );
            const minutes = String(this._selectedEndDate.getMinutes()).padStart(
                2,
                "0"
            );
            return `${hours}:${minutes}`;
        }

        // Format date as ISO but with local time (instead of UTC)
        getLocalISOString(date) {
            const year = date.getFullYear();
            const month = String(date.getMonth() + 1).padStart(2, "0");
            const day = String(date.getDate()).padStart(2, "0");
            const hours = String(date.getHours()).padStart(2, "0");
            const minutes = String(date.getMinutes()).padStart(2, "0");
            const seconds = String(date.getSeconds()).padStart(2, "0");
            const ms = String(date.getMilliseconds()).padStart(3, "0");

            return `${year}-${month}-${day}T${hours}:${minutes}:${seconds}.${ms}`;
        }

        // Convert the date to desired string format
        formatDateToString(date) {
            // Get local ISO string first
            const localIso = this.getLocalISOString(date);
            // Now remove punctuation as required
            return localIso
                .replaceAll(".", "")
                .replaceAll(":", "")
                .replaceAll("-", "");
        }

        setStartDateTime(date) {
            this._selectedStartDate = date;
            this.updateInputs();

            const formattedStartDate = this.formatDateToString(
                this._selectedStartDate
            );
            const formattedEndDate = this.formatDateToString(
                this._selectedEndDate
            );

            if (this._onChangeCallback) {
                this._onChangeCallback({
                    start: formattedStartDate,
                    end: formattedEndDate,
                });
            }

            this.dispatchEvent(
                new CustomEvent("change", {
                    detail: {
                        startValue: formattedStartDate,
                        endValue: formattedEndDate,
                        startRawDate: this._selectedStartDate,
                        endRawDate: this._selectedEndDate,
                    },
                    bubbles: true,
                })
            );
        }

        setEndDateTime(date) {
            this._selectedEndDate = date;
            this.updateInputs();

            const formattedStartDate = this.formatDateToString(
                this._selectedStartDate
            );
            const formattedEndDate = this.formatDateToString(
                this._selectedEndDate
            );

            if (this._onChangeCallback) {
                this._onChangeCallback({
                    start: formattedStartDate,
                    end: formattedEndDate,
                });
            }

            this.dispatchEvent(
                new CustomEvent("change", {
                    detail: {
                        startValue: formattedStartDate,
                        endValue: formattedEndDate,
                        startRawDate: this._selectedStartDate,
                        endRawDate: this._selectedEndDate,
                    },
                    bubbles: true,
                })
            );
        }

        updateInputs() {
            const startDateInput =
                this.shadowRoot.querySelectorAll(".date-input")[0];
            const endDateInput =
                this.shadowRoot.querySelectorAll(".date-input")[1];
            const startTimeInput =
                this.shadowRoot.querySelectorAll(".time-input")[0];
            const endTimeInput =
                this.shadowRoot.querySelector(".end-time-input");

            if (startDateInput && startTimeInput) {
                startDateInput.value = this.getStartDateString();
                startTimeInput.value = this.getStartTimeString();
            }

            if (endDateInput && endTimeInput) {
                endDateInput.value = this.getEndDateString();
                endTimeInput.value = this.getEndTimeString();
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
            <label>Start Date</label>
            <input type="date" class="date-input" value="${this.getStartDateString()}">
            <label>End Date</label>
            <input type="date" class="date-input" value="${this.getEndDateString()}">
          </div>
          <div class="input-group">
            <label>Start Time</label>
            <input type="time" class="time-input" value="${this.getStartTimeString()}">
            <label>End Time</label>
            <input type="time" class="end-time-input" value="${this.getEndTimeString()}">
          </div>
        </div>
      `;
        }

        // get value() {
        //     return {
        //         start: this.formatDateToString(this._selectedStartDate),
        //         end: this.formatDateToString(this._selectedEndDate),
        //     };
        // }

        get value() {
            const startFormatted = this.formatDateToString(
                this._selectedStartDate
            );
            const endFormatted = this.formatDateToString(this._selectedEndDate);
            return `${startFormatted} | ${endFormatted}`;
        }

        set startValue(newValue) {
            if (newValue instanceof Date) {
                this._selectedStartDate = new Date(newValue);
            } else {
                this._selectedStartDate = new Date(newValue);
            }
            this.updateInputs();
        }

        set endValue(newValue) {
            if (newValue instanceof Date) {
                this._selectedEndDate = new Date(newValue);
            } else {
                this._selectedEndDate = new Date(newValue);
            }
            this.updateInputs();
        }

        onChange(callback) {
            this._onChangeCallback = callback;
        }
    }

    // Register the web component
    customElements.define("date-time-picker", DateTimePicker);
})();

// Example usage:
// <date-time-picker start_value="2025-02-28T15:30:00" end_value="2025-02-28T16:30:00"></date-time-picker>
