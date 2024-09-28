import init, * as reverse_regex from "./reverse-regex/reverse_regex.js"


(async () => {
    await init();
    let pattern = "(lol|xd|dx){2,3}[0-9]{3,6}(anto|bob|john|mike)@(gmail|outlook|hotmail)[0-9]{1,10}\.(it|com|org|nas)";

    for(let i = 0; i < 1000; ++i) {
        let result = reverse_regex.generate(pattern);
        //console.log(result);
    }
})()

document.addEventListener('DOMContentLoaded', () => {
    const generateButton = document.getElementById("generate");
    const clearButton = document.getElementById("clear");
    
    const listDiv = document.getElementById("dynamicList");
    
    const inputRegex = document.getElementById("inputRegex");
    const inputNumber = document.getElementById("inputNumber");

    generateButton.addEventListener("click", (e) => {
        const pattern = inputRegex.value;
    
        for(let i = 0; i < inputNumber.value; ++i) {
            const result = reverse_regex.generate(pattern);
            const element = document.createElement("li");
            element.append(document.createTextNode(result));
            element.className = "list-group-item";

            listDiv.appendChild(element);        
        }
    });

    clearButton.addEventListener("click", (e) => {
        listDiv.innerHTML = "";
    });
});