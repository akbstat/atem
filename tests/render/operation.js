const classAttr = "class";
const div = "div"
const root = document.getElementById("root");
data.pages.forEach((page) => {
    // handle header
    const header = document.createElement(div);
    root.appendChild(header);
    header.setAttribute(classAttr, "header");
    const headerLeft = document.createElement(div);
    headerLeft.setAttribute(classAttr, "left");
    header.appendChild(headerLeft);
    const projectNode = document.createElement(div);
    const formNode = document.createElement(div);
    projectNode.innerText = "Project Name: AK105-302";
    formNode.innerText = `Form: ${page.module_name}`;
    headerLeft.appendChild(projectNode);
    headerLeft.appendChild(formNode);

});


