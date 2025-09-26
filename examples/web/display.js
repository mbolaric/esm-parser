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
        this.currentData = undefined;
        this.verticalMenu = document.getElementById("shell.content.left.body.menu");
        this.dataContent = document.getElementById("data-content");
        this.contentHeader = document.getElementById("shell.content.main.header.label");
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
                }
            } else {
                data = this.currentData.transferResParams.filter((item) => item.typeId === key);
                if (data) {
                    console.log(key, gen, data);
                    this.contentHeader.textContent = `File part: ${key}`;
                    this.applyDataContent(data);
                }
            }
        }
    };

    addCardMenu = (data, title, key, gen) => {
        if (data[key]) {
            this.addMenuElement(title, key, gen);
        }
    };

    clean = () => {
        this.verticalMenu.innerHTML = "";
        this.dataContent.innerHTML = "";
        this.currentData = undefined;
    };

    applyDataContent = (data) => {
        const formattedData = JSON.stringify(data, null, 2);
        this.dataContent.textContent = formattedData;
        console.log("Success! Parsed data:", formattedData);
    };

    applyData = (rootEl, data) => {
        this.clean();
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
            const uniqueTypes = [...new Set(data.map(item => item.typeId))];
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

    processSecondGenerationCard = (dataType, data) => {
        const gen1 = data.gen1;
        const gen2 = data.gen2;
        const rootData = gen1 || gen2;

        this.processCardAllGenHeader(rootData);
        if (gen1) {
            this.processCardGen1(gen1);
        }
        if (gen2) {
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
        }
    };
}
