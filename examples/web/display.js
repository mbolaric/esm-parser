export const Generation = {
    FirstGeneration: "FirstGeneration",
    SecondGeneration: "SecondGeneration",
};

export const CardGeneration = {
    FirstGeneration: "Gen1",
    SecondGeneration: "Gen2",
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
    CardNotes: "cardNotes"
};

export class DisplayData {
    constructor() {
        this.verticalMenu = document.getElementById("shell.content.left.body.menu");
        this.dataContent = document.getElementById("data-content");
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
    };

    addCardMenu = (data, title, key, gen) => {
        if (data[key]) {
            this.addMenuElement(title, key, gen);
        }
    };

    applyData = (rootEl, data) => {
        const header = data.header;
        console.log(header.generation);
        console.log(header.dataType);
        switch (header.generation) {
            case Generation.FirstGeneration:
                if (header.dataType === "Card") {
                    this.processFirstGenerationCard(header.dataType, data.cardDataResponses);
                } else {
                }
                break;
            case Generation.SecondGeneration:
                if (header.dataType === "Card") {
                    this.processSecondGenerationCard(header.dataType, data.cardDataResponses);
                } else {
                }
                break;
        }

        // FIXME: only for now ...
        const formattedData = JSON.stringify(data, null, 2);
        this.dataContent.textContent = formattedData;
        console.log("Success! Parsed data:", formattedData);
    };

    processFirstGenerationCard = (dataType, data) => {
        console.log(dataType);
    };

    processSecondGenerationCard = (dataType, data) => {
        const gen1 = data.gen1;
        const gen2 = data.gen2;
        const rootData = gen1 || gen2;
        this.addMenuHeader(rootData.applicationIdentification.typeOfTachographCardId);
        this.addMenuElement("Identification", DataParts.Identification, rootData.cardGeneration);
        this.addMenuElement("IC", DataParts.IC, rootData.cardGeneration);
        this.addMenuElement("ICC", DataParts.ICC, rootData.cardGeneration);
        this.addMenuDivider();
        if (gen1) {
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
