export const Generation = {
    FirstGeneration: "FirstGeneration",
    SecondGeneration: "SecondGeneration",
};

export const CardGeneration = {
    FirstGeneration: "Gen1",
    SecondGeneration: "Gen2",
};

export const VUGeneration = {
    FirstGeneration: "VUGen1",
    SecondGeneration: "VUGen2",
};

export const DataParts = {
    Identification: "identification",
    IC: "cardChipIdentification",
    ICC: "cardIccIdentification",
    DrivingLicenceInformation: "drivingLicenceInformation",
    VehiclesUsed: "vehiclesUsed",
    EventsData: "eventsData",
    FaultsData: "faultsData",
    Places: "places",
    CurrentUsage: "currentUsage",
    DriverActivityData: "driverActivityData",
    SpecificConditions: "specificConditions",
    ControlActivityData: "controlActivityData",
    CardCertificate: "cardCertificate",
    CACertificate: "caCertificate",
    CardDownload: "cardDownload",
    CardNotes: "cardNotes",
};

export class DisplayData {
    constructor() {
        this.fileName = undefined;
        this.currentData = undefined;
        this.currentPart = undefined;
        this.verticalMenu = document.getElementById("shell.content.left.body.menu");
        this.dataContent = document.getElementById("data-content");
        this.contentHeader = document.getElementById("shell.content.main.header.label");
        this.buttionExportPart = document.getElementById("button-export-part");
        this.buttionExportAll = document.getElementById("button-export-all");

        this.buttionExportPart.addEventListener("pointerup", this.onExport);
        this.buttionExportAll.addEventListener("pointerup", this.onExport);
    }

    exportData = (key, data) => {
        if (data) {
            const json = JSON.stringify(data, null, 4);
            const blob = new Blob([json], { type: "application/json" });
            const url = URL.createObjectURL(blob);
            const a = document.createElement("a");
            a.href = url;
            a.download = this.fileName.replace(/.DDD/i, `-${key}.json`);
            a.click();
        }
    };

    onExport = (e) => {
        switch (e.currentTarget.id) {
            case "button-export-part":
                if (this.currentPart) {
                    this.exportData(this.currentPart.key, this.currentPart.data);
                }
                break;
            case "button-export-all":
                if (this.currentData) {
                    this.exportData("all", this.currentData);
                }
                break;
        }
    };

    disableAllButtons = () => {
        this.setButtonState(this.buttionExportPart, true);
        this.setButtonState(this.buttionExportAll, true);
    };

    setButtonState(buttonEl, disable) {
        if (disable) {
            buttonEl.classList.add("disabled");
        } else {
            buttonEl.classList.remove("disabled");
        }
    }

    createElement = (getChilds) => {
        const el = document.createElement("div");
        el.innerHTML = getChilds();
        return el;
    };

    addMenuHeader = (typeOfTachographCardId) => {
        let el = this.createElement(() => {
            return typeOfTachographCardId;
        });
        el.classList.add("menu-header");
        this.verticalMenu.appendChild(el);
    };

    addMenuElement = (title, key, gen) => {
        const el = document.createElement("div");
        el.classList.add("menu-row");
        el.dataset.key = key;
        el.dataset.gen = gen;
        el.addEventListener("mouseup", this.onMenuClick);
        const aEl = document.createElement("a");
        aEl.innerText = title;
        el.appendChild(aEl);
        this.verticalMenu.appendChild(el);
    };

    addMenuDivider = () => {
        const el = document.createElement("div");
        el.classList.add("menu-divider");
        this.verticalMenu.appendChild(el);
    };

    onMenuClick = (e) => {
        console.log("onMenuClick", e);
        if (this.currentData) {
            const dataset = e.currentTarget.dataset;
            let key = dataset.key;
            let gen = dataset.gen;

            let data;
            if (gen === CardGeneration.FirstGeneration || gen === CardGeneration.SecondGeneration) {
                if (gen === CardGeneration.FirstGeneration) {
                    data = this.currentData.cardDataResponses.gen1 || this.currentData.cardDataResponses;
                } else {
                    data = this.currentData.cardDataResponses.gen2;
                }

                if (data) {
                    const dataKey = data[key];
                    console.log(key, gen, dataKey);
                    this.contentHeader.textContent = `File part: ${key}`;
                    this.applyDataContent(dataKey);
                    this.currentPart = {
                        key,
                        data: dataKey
                    };
                }
            } else {
                data = this.currentData.transferResParams.filter((item) => item.typeId === key);
                if (data) {
                    console.log(key, gen, data);
                    this.contentHeader.textContent = `File part: ${key}`;
                    this.applyDataContent(data);
                    this.currentPart = {
                        key,
                        data
                    };
                }
            }
            this.setButtonState(this.buttionExportPart, false);
        }
    };

    addCardMenu = (data, title, key, gen) => {
        if (data[key]) {
            this.addMenuElement(title, key, gen);
        }
    };

    clean = () => {
        this.fileName = undefined;
        this.verticalMenu.innerHTML = "";
        this.dataContent.innerHTML = "";
        this.currentData = undefined;
        this.currentPart = undefined;
        this.disableAllButtons();
    };

    applyDataContent = (data) => {
        const formattedData = JSON.stringify(data, null, 2);
        this.dataContent.textContent = formattedData;
        console.log("Success! Parsed data:", formattedData);
        this.setButtonState(this.buttionExportAll, false);
    };

    mapToObject = (map) => {
      const obj = {};
      for (const [key, value] of map.entries()) {
        obj[key] = value instanceof Map ? mapToObject(value) : value;
      }
      return obj;
    }

    applyData = (rootEl, data, fileName) => {
        this.clean();
        this.fileName = fileName;
        if (data.cardDataResponses && data.cardDataResponses.dataFiles) {
            data.cardDataResponses.dataFiles = this.mapToObject(data.cardDataResponses.dataFiles);
        }
        if (data.transferResParams && data.data.transferResParams.dataFiles) {
            data.transferResParams.dataFiles = this.mapToObject(data.transferResParams.dataFiles);
        }
        this.currentData = data;
        const header = data.header;
        console.log(header.generation);
        console.log(header.dataType);
        switch (header.generation) {
            case Generation.FirstGeneration:
                if (header.dataType === "Card") {
                    this.processFirstGenerationCard(header.dataType, data.cardDataResponses);
                } else {
                    this.processAllGenerationVU(header.dataType, data.transferResParams, VUGeneration.FirstGeneration);
                }
                break;
            case Generation.SecondGeneration:
                if (header.dataType === "Card") {
                    this.processSecondGenerationCard(header.dataType, data.cardDataResponses);
                } else {
                    this.processAllGenerationVU(header.dataType, data.transferResParams, VUGeneration.SecondGeneration);
                }
                break;
        }

        this.applyDataContent(data);
    };

    addVUMenu = (data, title, key, gen) => {
        if (data[key]) {
            this.addMenuElement(title, key, gen);
        }
    };

    processAllGenerationVU = (dataType, data, gen) => {
        if (data) {
            this.addMenuHeader(`${gen}`);
            const uniqueTypes = [...new Set(data.map((item) => item.typeId))];
            uniqueTypes.forEach((item) => {
                this.addMenuElement(item, item, gen);
            });
        }
    };

    processCardAllGenHeader = (data) => {
        this.addMenuHeader(data.applicationIdentification.typeOfTachographCardId);
        this.addMenuElement("Identification", DataParts.Identification, data.cardGeneration);
        this.addMenuElement("IC", DataParts.IC, data.cardGeneration);
        this.addMenuElement("ICC", DataParts.ICC, data.cardGeneration);
        this.addMenuDivider();
    };

    processFirstGenerationCard = (dataType, data) => {
        this.processCardAllGenHeader(data);
        this.processCardGen1(data);
    };

    processCardGen1 = (gen1) => {
        this.addMenuHeader("Card Generation 1");
        this.addCardMenu(gen1, "DrivingLicenceInformation", DataParts.DrivingLicenceInformation, CardGeneration.FirstGeneration);
        this.addCardMenu(gen1, "VehiclesUsed", DataParts.VehiclesUsed, CardGeneration.FirstGeneration);
        this.addCardMenu(gen1, "EventsData", DataParts.EventsData, CardGeneration.FirstGeneration);
        this.addCardMenu(gen1, "FaultsData", DataParts.FaultsData, CardGeneration.FirstGeneration);
        this.addCardMenu(gen1, "Places", DataParts.Places, CardGeneration.FirstGeneration);
        this.addCardMenu(gen1, "CurrentUsage", DataParts.CurrentUsage, CardGeneration.FirstGeneration);
        this.addCardMenu(gen1, "DriverActivityData", DataParts.DriverActivityData, CardGeneration.FirstGeneration);
        this.addCardMenu(gen1, "SpecificConditions", DataParts.SpecificConditions, CardGeneration.FirstGeneration);
        this.addCardMenu(gen1, "ControlActivityData", DataParts.ControlActivityData, CardGeneration.FirstGeneration);
        this.addCardMenu(gen1, "CardCertificate", DataParts.CardCertificate, CardGeneration.FirstGeneration);
        this.addCardMenu(gen1, "CACertificate", DataParts.CACertificate, CardGeneration.FirstGeneration);
    };

    processCardGen2 = (gen2) => {
        this.addMenuHeader("Card Generation 2");
        this.addCardMenu(gen2, "DrivingLicenceInformation", DataParts.DrivingLicenceInformation, CardGeneration.SecondGeneration);
        this.addCardMenu(gen2, "VehiclesUsed", DataParts.VehiclesUsed, CardGeneration.SecondGeneration);
        this.addCardMenu(gen2, "EventsData", DataParts.EventsData, CardGeneration.SecondGeneration);
        this.addCardMenu(gen2, "FaultsData", DataParts.FaultsData, CardGeneration.SecondGeneration);
        this.addCardMenu(gen2, "Places", DataParts.Places, CardGeneration.SecondGeneration);
        this.addCardMenu(gen2, "CurrentUsage", DataParts.CurrentUsage, CardGeneration.SecondGeneration);
        this.addCardMenu(gen2, "DriverActivityData", DataParts.DriverActivityData, CardGeneration.SecondGeneration);
        this.addCardMenu(gen2, "SpecificConditions", DataParts.SpecificConditions, CardGeneration.SecondGeneration);
        this.addCardMenu(gen2, "ControlActivityData", DataParts.ControlActivityData, CardGeneration.SecondGeneration);
        this.addCardMenu(gen2, "CardCertificate", DataParts.CardCertificate, CardGeneration.SecondGeneration);
        this.addCardMenu(gen2, "CACertificate", DataParts.CACertificate, CardGeneration.SecondGeneration);
    };

    processSecondGenerationCard = (dataType, data) => {
        const gen1 = data.gen1;
        const gen2 = data.gen2;
        const rootData = gen1 || gen2;

        this.processCardAllGenHeader(rootData);
        if (gen1) {
            this.processCardGen1(gen1);
        }
        if (gen2) {
            this.processCardGen2(gen2);
        }
    };
}
