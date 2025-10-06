import { verify } from "../../pkg/esm_parser.js";

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
    ApplicationIdentification: "applicationIdentification",
    ApplicationIdentificationV2: "applicationIdentificationV2",
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
    CardSignCertificate: "cardSignCertificate",
    LinkCertificate: "linkCertificate",
    CardDownload: "cardDownload",
    VehicleUnitsUsed: "vehicleUnitsUsed",
    GnssPlaces: "gnssPlaces"
};

const ec_pk_gen2 = new Uint8Array([
        127, 33, 129, 201, 127, 78, 129, 130, 95, 41, 1, 0, 66, 8, 253, 69, 67, 32, 1, 255, 255, 1, 95, 76, 7, 255, 83, 77, 82, 68,
        84, 13, 127, 73, 78, 6, 9, 43, 36, 3, 3, 2, 8, 1, 1, 7, 134, 65, 4, 8, 192, 78, 57, 38, 200, 222, 133, 84, 66, 64, 205, 228,
        13, 171, 112, 210, 180, 126, 15, 131, 118, 37, 34, 215, 176, 184, 84, 59, 155, 41, 220, 128, 229, 198, 123, 130, 166, 45, 85,
        227, 72, 58, 180, 176, 10, 36, 194, 162, 86, 108, 55, 134, 121, 122, 26, 5, 40, 34, 171, 75, 241, 242, 146, 95, 32, 8, 253,
        69, 67, 32, 1, 255, 255, 1, 95, 37, 4, 91, 33, 176, 0, 95, 36, 4, 155, 143, 174, 128, 95, 55, 64, 101, 198, 42, 193, 61, 237,
        20, 127, 168, 209, 209, 26, 143, 91, 242, 207, 158, 149, 219, 27, 67, 210, 83, 180, 139, 97, 91, 47, 231, 11, 63, 216, 42,
        168, 211, 61, 39, 240, 244, 215, 54, 124, 4, 144, 59, 187, 230, 55, 91, 100, 58, 25, 197, 184, 61, 25, 252, 116, 133, 219,
        71, 108, 112, 103,
    ]);

const ec_pk_gen1 = new Uint8Array([
        253, 69, 67, 32, 0, 255, 255, 1, 233, 128, 118, 58, 68, 74, 149, 37, 10, 149, 135, 130, 209, 213, 74, 207, 195, 35, 210,
        95, 57, 70, 184, 22, 233, 47, 207, 157, 50, 180, 42, 38, 19, 209, 163, 99, 180, 228, 53, 50, 160, 38, 104, 99, 41, 200,
        150, 99, 204, 192, 1, 247, 39, 130, 6, 182, 171, 101, 173, 40, 113, 132, 138, 104, 15, 106, 87, 216, 253, 161, 215, 130,
        201, 181, 129, 41, 3, 234, 91, 102, 226, 169, 190, 29, 133, 189, 208, 253, 174, 118, 164, 96, 136, 215, 26, 97, 118, 177,
        246, 169, 132, 25, 16, 4, 36, 220, 86, 208, 132, 106, 163, 200, 67, 144, 211, 81, 122, 15, 17, 146, 222, 223, 247, 64,
        146, 76, 219, 167, 0, 0, 0, 0, 0, 1, 0, 1,
    ]);

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
            const json = JSON.stringify(data, this.mapToObject, 4);
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
        const formattedData = JSON.stringify(data, this.mapToObject, 2);
        this.dataContent.textContent = formattedData;
        console.log("Success! Parsed data:", formattedData);
        this.setButtonState(this.buttionExportAll, false);
    };

    mapToObject = (key, value) => {
        if (value instanceof Map) {
            return Object.fromEntries(value);
        } else {
            return value;
        }
    };

    verifyData = (data, ec_pk) => {
        setTimeout(() => {
            let resutl = verify(data, ec_pk);
            console.log("Verify: ", resutl);
        }, 0);
    };

    applyData = (rootEl, data, fileName) => {
        this.clean();
        this.fileName = fileName;
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
        this.verifyData(data.dataFiles, ec_pk_gen1);
    };

    processCardGen1 = (gen1) => {
        this.addMenuHeader("Card Generation 1");
        this.addCardMenu(gen1, "ApplicationIdentification", DataParts.ApplicationIdentification, CardGeneration.FirstGeneration);
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
        this.addCardMenu(gen2, "ApplicationIdentification", DataParts.ApplicationIdentification, CardGeneration.SecondGeneration);
        this.addCardMenu(gen2, "ApplicationIdentificationV2", DataParts.ApplicationIdentificationV2, CardGeneration.SecondGeneration);
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
        this.addCardMenu(gen2, "CardSignCertificate", DataParts.CardSignCertificate, CardGeneration.SecondGeneration);
        this.addCardMenu(gen2, "CACertificate", DataParts.CACertificate, CardGeneration.SecondGeneration);
        this.addCardMenu(gen2, "LinkCertificate", DataParts.LinkCertificate, CardGeneration.SecondGeneration);
        this.addCardMenu(gen2, "CardDownload", DataParts.CardDownload, CardGeneration.SecondGeneration);
        this.addCardMenu(gen2, "VehicleUnitsUsed", DataParts.VehicleUnitsUsed, CardGeneration.SecondGeneration);
        this.addCardMenu(gen2, "GnssPlaces", DataParts.GnssPlaces, CardGeneration.SecondGeneration);
    };

    processSecondGenerationCard = (dataType, data) => {
        const gen1 = data.gen1;
        const gen2 = data.gen2;
        const rootData = gen1 || gen2;

        this.processCardAllGenHeader(rootData);
        if (gen1) {
            this.processCardGen1(gen1);
            this.verifyData(gen1.dataFiles, ec_pk_gen1);
        }
        if (gen2) {
            this.processCardGen2(gen2);
            this.verifyData(gen2.dataFiles, ec_pk_gen2);
        }
    };
}
