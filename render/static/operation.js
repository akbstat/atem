const classAttr = "class";
const div = "div"
const input = "input"
const root = document.getElementById("root");
data.forms.forEach((form) => {
    // handle header
    const header = document.createElement(div);
    header.setAttribute(classAttr, "header");
    const headerLeft = document.createElement(div);
    headerLeft.setAttribute(classAttr, "left");
    header.appendChild(headerLeft);
    const projectNode = document.createElement(div);
    const formNode = document.createElement(div);
    projectNode.innerText = `Project Name: ${data.name}`;
    formNode.innerText = `Form: ${form.label}`;
    headerLeft.appendChild(projectNode);
    headerLeft.appendChild(formNode);
    const headerAnno = document.createElement(div);

    // rebuild group anno
    const groups = form.group.map(group => {
        const anno = form.anno.filter(anno => anno.group === group.id);
        return {
            id: group.id,
            name: group.name,
            label: group.label,
            anno: anno
        }
    });
    for (const group of groups) {
        const groupNode = document.createElement(div);
        groupNode.setAttribute(classAttr, "right");
        const labelNode = document.createElement(div);
        labelNode.setAttribute(classAttr, `annotation-${group.id} domain mapping`);
        labelNode.innerText = `${group.name}(${group.label})`;
        groupNode.appendChild(labelNode);
        for (const anno of group.anno) {
            const annoNode = document.createElement(div);
            if (anno.assign) {
                annoNode.setAttribute(classAttr, `annotation-${anno.group} assign`);
            } else {
                annoNode.setAttribute(classAttr, `annotation-${anno.group} mapping`);
            }
            // annoNode.setAttribute(classAttr, `annotation-${group.id} mapping`);
            annoNode.innerText = anno.value;
            groupNode.appendChild(annoNode);
        }
        header.appendChild(groupNode);
    }
    header.appendChild(headerAnno);
    root.appendChild(header.cloneNode(true));

    // handle items
    let itemsNode = document.createElement(div);
    itemsNode.setAttribute(classAttr, "items");
    for (const item of form.items) {
        itemsNode.appendChild(document.createElement("hr"));
        const itemNode = document.createElement(div);
        itemNode.setAttribute(classAttr, "item");

        // question node
        const questionNode = document.createElement(div);
        questionNode.setAttribute(classAttr, "question");

        // for multi selection
        if (item.itemType === "checkbox") {
            const checkboxNode = document.createElement(input);
            checkboxNode.setAttribute("type", "checkbox");
            checkboxNode.style.marginTop = "1px";
            questionNode.appendChild(checkboxNode);
        }

        const questionContnet = document.createElement(div);
        questionContnet.setAttribute(classAttr, "question-content");
        questionContnet.innerText = item.label;
        questionNode.appendChild(questionContnet);
        for (const anno of item.anno) {
            const annoNode = document.createElement(div);
            if (anno.assign) {
                annoNode.setAttribute(classAttr, `annotation-${anno.group} assign`);
            } else {
                annoNode.setAttribute(classAttr, `annotation-${anno.group} mapping`);
            }
            annoNode.innerText = anno.value;
            questionContnet.appendChild(annoNode);
        }
        if (item.unit) {
            const unitNode = document.createElement(div);
            unitNode.setAttribute(classAttr, "unit")
            unitNode.innerText = item.unit.name;
            questionNode.appendChild(unitNode);
            for (const anno of item.unit.anno) {
                const annoNode = document.createElement(div);
                if (anno.assign) {
                    annoNode.setAttribute(classAttr, `annotation-${anno.group} assign`);
                } else {
                    annoNode.setAttribute(classAttr, `annotation-${anno.group} mapping`);
                }
                annoNode.innerText = anno.value;
                unitNode.appendChild(annoNode);
            }
        }
        itemNode.appendChild(questionNode);
        // option node
        if (item.itemType === "selection") {
            for (const option of item.options) {
                const optionNode = document.createElement(div);
                optionNode.setAttribute(classAttr, "option");
                const checkboxNode = document.createElement(input);
                checkboxNode.setAttribute("type", "checkbox");
                optionNode.appendChild(checkboxNode);
                // optionNode.appendChild(document.createTextNode(option.label));
                const optionLabel = document.createElement(div)
                optionLabel.innerText = option.label;
                optionLabel.style.justifyContent = "center";
                optionLabel.style.display = "flex";
                optionLabel.style.alignItems = "center";
                optionNode.appendChild(optionLabel);
                if (!option.anno) {
                    continue;
                }
                for (const optionAnno of option.anno) {
                    const annoNode = document.createElement(div);
                    if (optionAnno.assign) {
                        annoNode.setAttribute(classAttr, `annotation-${optionAnno.group} assign`);
                    } else {
                        annoNode.setAttribute(classAttr, `annotation-${optionAnno.group} mapping`);
                    }
                    annoNode.innerText = optionAnno.value;
                    optionNode.appendChild(annoNode);
                }
                itemNode.appendChild(optionNode);
            }
        }
        itemsNode.appendChild(itemNode);
        if (item.page_break) {
            root.appendChild(itemsNode);
            const breaker = document.createElement("div");
            breaker.setAttribute(classAttr, "break-page");
            root.appendChild(breaker);
            root.appendChild(header.cloneNode(true));
            itemsNode = document.createElement(div);
            itemsNode.setAttribute(classAttr, "items");
            itemsNode.setAttribute(classAttr, "items");
        }
    }
    itemsNode.appendChild(document.createElement("hr"));
    root.appendChild(itemsNode);

    const breaker = document.createElement("div");
    breaker.setAttribute(classAttr, "break-page");
    root.appendChild(breaker);
});


