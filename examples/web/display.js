export const Generation = {
  FirstGeneration: "FirstGeneration",
  SecondGeneration: "SecondGeneration",
};

export class DisplayData {
  constructor() {
    this.leftRoot = document.getElementById("link-container");
  }

  createElement(parent, getChilds) {
    const el = document.createElement("div");
    el.innerHTML = getChilds();
    parent.appendChild(el);
    return el;
  }

  addHeader(typeOfTachographCardId) {
    this.createElement(this.leftRoot, () => {
      return typeOfTachographCardId;
    });
  }

  addMenuElement(appIdentification, identification) {
    const el = this.createElement(this.leftRoot, () => {
      return appIdentification.typeOfTachographCardId;
    });
    el.addEventListener("onmouseup", this.onMenuClick);
  }

  onMenuClick(e) {
    console.log("onMenuClick");
  }

  applyData(rootEl, data) {
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

    console.log("Success! Parsed data:", JSON.stringify(data, null, 2));
  }

  processFirstGenerationCard(dataType, data) {
    console.log(dataType);
  }

  processSecondGenerationCard(dataType, data) {
    const gen1 = data.gen1;
    const gen2 = data.gen2;
    const rootData = gen1 || gen2;
    this.addHeader(rootData.applicationIdentification.typeOfTachographCardId);
    if (gen1) {
      console.log(gen1.applicationIdentification);
      this.addMenuElement(gen1.applicationIdentification, gen1.identification);
    }
    if (gen2) {
      console.log(gen1.applicationIdentification);
    }
  }
}
