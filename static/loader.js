

const fileSelector = document.getElementById('file-selector');
fileSelector.addEventListener('change', (event) => {


    const fileList = event.target.files;
    if (fileList.length == 1){
        for (const file of fileList) {
            readFile(file)
        }
    }else{
        alert('you must select only one file')
    }

});

function readFile(file) {
    const reader = new FileReader();
    reader.addEventListener('load', (event) => {

        console.log(event.target.result)
        let res = window.greet( event.target.result)
       console.log( res);
    });
    reader.readAsText(file);
}
