(async () => {
	var printers = await fetch("/printers").then((x) => x.json());
	var selectElement = document.getElementById("printerName");

	printers.forEach((element) => {
		selectElement.add(new Option(element));
	});
})();

document
	.getElementById("printerForm")
	.addEventListener("submit", onSubmit, false);

function onSubmit(e) {
	e.preventDefault();

	console.log("Printing...");

	var input = document.getElementById("documentFile");
	let file = input.files[0];

	console.log(file);

	var data = new FormData();
	data.append(file.name, file);

	fetch("/print", {
		method: "POST",
		body: data,
	});
}
