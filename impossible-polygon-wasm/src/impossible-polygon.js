const rust = import('../pkg/impossible_polygon_wasm');
import iro from '@jaames/iro';

let colorPalette = [
    "#D27B8F", "#D5A18E", "#98B06F",
    "#DBFF76", "#00F2F2", "#096B72",
    "#7F675B", "#FFD97D", "#187795",
    "#717744", "#84A98C", "#ADD9F4",
    "#ADA8B6", "#B3B492", "#8B87B7",
    "#D1F5BE", "#FB9F89", "#B46A61",
    "#FFEEDB"
];

export function startDownload() {
    let svg = document.getElementsByTagName("svg")[0];
    svg.setAttribute("xmlns", "http://www.w3.org/2000/svg");
    let svgData = svg.outerHTML;
    let preface = '<?xml version="1.0" standalone="no"?>\r\n';
    let svgBlob = new Blob([preface, svgData], {type: "image/svg+xml;charset=utf-8"});
    let svgUrl = URL.createObjectURL(svgBlob);
    let downloadLink = document.createElement("a");
    downloadLink.href = svgUrl;
    downloadLink.download = "penrose-polygon.svg";
    document.body.appendChild(downloadLink);
    downloadLink.click();
    document.body.removeChild(downloadLink);
}

export function generatePolygon(n, thickness, perspective) {
    console.log("rendering..");

    let output = document.getElementById("output");
    output.innerHTML = loaded_rust.generate_penrose_polygon(parseInt(n), false, thickness, perspective, colorPalette);

    let index = 0;
    for (let polygon of document.getElementsByTagName("polygon")) {
        let _index = index;
        polygon.onclick = () => {
            console.log("You clicked on " + _index);
            showColorPicker(_index, polygon)
        };
        index += 1;
    }
}

let colorPickerContainer = $('#color-picker-container');
colorPickerContainer.hide();
let colorPicker = iro.ColorPicker('#color-picker-container', {
    width: 200,
    height: 300,
    color: "#f00"
});

let currentPolygon = null;
let currentColorIndex = null;

colorPicker.on("color:change", () => {
    let color = colorPicker.color.hexString;
    currentPolygon.setAttribute("fill", color);
    colorPalette[currentColorIndex] = color;
    console.log("current index " + currentColorIndex);
});


$('#close-color-picker').click(() => {
    currentPolygon = null;
    currentColorIndex = null;
    colorPickerContainer.hide();
});

function showColorPicker(index, element) {
    currentPolygon = element;
    currentColorIndex = index;
    colorPickerContainer.show();
}

let loaded_rust = null;

const initialize = async () => {
    loaded_rust = await rust;

    let edges = 3, thickness = 0.7, perspective = 0.1;

    $('#edges-slider')
        .slider({
            min: 3, max: 16, start: edges, step: 1,
            onMove: (e) => {
                if (e !== edges) {
                    edges = e;
                    generatePolygon(edges, thickness, perspective);
                }
            }
        });
    $('#thickness-slider')
        .slider({
            min: 0, max: 1, start: thickness, step: 0,
            onMove: (e) => {
                thickness = 1 - e;
                generatePolygon(edges, thickness, perspective);
            }
        });
    $('#perspective-slider')
        .slider({
            min: 0, max: 1, start: perspective, step: 0,
            onMove: (e) => {
                perspective = 1 - e;
                generatePolygon(edges, thickness, perspective);
            }
        });
    $('#download-button').click(startDownload);


    generatePolygon(edges, thickness, perspective);
};

initialize().then(() => console.log("wasm initialized"));