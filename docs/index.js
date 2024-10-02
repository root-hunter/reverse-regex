import init, * as reverse_regex from "./reverse-regex/reverse_regex.js"

document.addEventListener('DOMContentLoaded', () => {
    const generateButton = document.getElementById("generate");
    const clearButton = document.getElementById("clear");
    const dowloadCSVButton = document.getElementById("download-csv");
    const copyAll = document.getElementById("copy-all");

    const listDiv = document.getElementById("results-list");

    const inputRegex = document.getElementById("input-pattern");
    const inputNumber = document.getElementById("input-number");

    const toastLiveExample = document.getElementById('liveToast');
    const toastBootstrap = bootstrap.Toast.getOrCreateInstance(toastLiveExample);
    const toastText = document.getElementById('toast-text');

    generateButton.addEventListener("click", (e) => {
        const pattern = inputRegex.value;

        for (let i = 0; i < inputNumber.value; ++i) {
            const result = reverse_regex.generate(pattern);
            let index = listDiv.children.length;

            listDiv.insertAdjacentHTML("beforeend", `
             <li class="list-group-item"> 
                <div class="row">
                  <div class="col-10 text-wrap" style="word-break: break-word; overflow-wrap: break-word;">
                    <p id="result-${index}">${result}</p>
                  </div>
                  <div class="col-2">
                    <button id="copy-result-${index}" class="btn"><strong>Copy</strong></button>
                  </div>
                </div>
              </li>
            `);

            const element = document.getElementById(`copy-result-${index}`);

            element.addEventListener("click", () => {
                const value = document.getElementById(`result-${index}`).innerText;
                navigator.clipboard.writeText(value);
       
                toastText.innerText = `Result copied to clipboard (${value})`;
                toastBootstrap.show();
            });
        }
    });

    clearButton.addEventListener("click", (e) => {
        listDiv.innerHTML = "";
    });

    dowloadCSVButton.addEventListener("click", (e) => {
        let out = "";

        for(let index = 0; index < listDiv.children.length; ++index) {
            const value = document.getElementById(`result-${index}`).innerText;

            out += `${value}\n`;
        }
        navigator.clipboard.writeText(out);
        
        const element = document.createElement('a');
        element.setAttribute('href', 'data:text/csv;charset=utf-8,' + encodeURIComponent(out));
        const filename = prompt("Please enter filename", `results_${Date.now()}`);

        if(filename !== null) {
            const full_filename = `${filename}.csv`;
            element.setAttribute('download', full_filename);
      
            element.style.display = 'none';
            document.body.appendChild(element);
          
            element.click();
          
            document.body.removeChild(element);

            toastText.innerText = `CSV file generated ${full_filename} (total: ${listDiv.children.length})`;
            toastBootstrap.show();
        }
    });

    copyAll.addEventListener("click", (e) => {
        let out = "";

        for(let index = 0; index < listDiv.children.length; ++index) {
            const value = document.getElementById(`result-${index}`).innerText;

            out += `${value}\n`;
        }
        navigator.clipboard.writeText(out);
        toastText.innerText = `All results copied to clipboard (total: ${listDiv.children.length})`;
        toastBootstrap.show();
    });

    (async () => {
        await init();
        generateButton.disabled = false;
    })()
});