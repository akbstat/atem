import { useAtemStore } from "@/stores/counter";
import axios from "axios";
import { storeToRefs } from 'pinia';

export async function getProject(): Promise<Project> {
    return {
        id: 1,
        name: "XX001-001",
        forms: [
            {
                id: 1,
                name: "AE",
                label: "不良事件/严重不良事件评估",
                group: [],
                anno: [],
                items: [
                    {
                        id: 1,
                        name: "AEYN",
                        label: "是否发生不良事件？",
                        itemType: "selection",
                        anno: [],
                        options: [
                            {
                                id: 1,
                                label: "是",
                                anno: [],
                            },
                            {
                                id: 2,
                                label: "否",
                                anno: [],
                            }
                        ],
                    },
                    {
                        id: 2,
                        name: "AETERM",
                        label: "不良事件名称",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 3,
                        name: "AESTDAT",
                        label: "开始日期",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 4,
                        name: "AEENDAT",
                        label: "结束日期",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 5,
                        name: "AELABEL1",
                        label: "符合的AESI标准（多选）：",
                        itemType: "label",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 6,
                        name: "AESI1",
                        label: "严重感染",
                        itemType: "checkbox",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 7,
                        name: "AESI2",
                        label: "全身性或广泛超敏反应",
                        itemType: "checkbox",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 8,
                        name: "AESI3",
                        label: "睑缘炎（重度或严重或持续>=4周）",
                        itemType: "checkbox",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 9,
                        name: "AESI4",
                        label: "任何类型的结膜炎 （重度或严重或持续>=4周）",
                        itemType: "checkbox",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 10,
                        name: "AESER",
                        label: "是否为严重不良事件(SAE)？",
                        itemType: "selection",
                        anno: [],
                        options: [
                            {
                                id: 3,
                                label: "是",
                                anno: [],
                            },
                            {
                                id: 4,
                                label: "否",
                                anno: [],
                            }
                        ],
                    },
                    {
                        id: 11,
                        name: "AELABEL2",
                        label: "符合的严重不良事件标准：",
                        itemType: "label",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 12,
                        name: "AESDTH",
                        label: "导致死亡",
                        itemType: "selection",
                        anno: [],
                        options: [
                            {
                                id: 5,
                                label: "是",
                                anno: [],
                            },
                            {
                                id: 6,
                                label: "否",
                                anno: [],
                            }
                        ],
                    },
                    {
                        id: 13,
                        name: "AESLIFE",
                        label: "危及生命",
                        itemType: "selection",
                        anno: [],
                        options: [
                            {
                                id: 7,
                                label: "是",
                                anno: [],
                            },
                            {
                                id: 8,
                                label: "否",
                                anno: [],
                            }
                        ],
                    },
                    {
                        id: 14,
                        name: "AESHOSP",
                        label: "导致住院或住院时间延长",
                        itemType: "selection",
                        anno: [],
                        options: [
                            {
                                id: 9,
                                label: "是",
                                anno: [],
                            },
                            {
                                id: 10,
                                label: "否",
                                anno: [],
                            }
                        ],
                    },
                    {
                        id: 15,
                        name: "AESDISAB",
                        label: "导致永久或严重的残疾/能力丧失",
                        itemType: "selection",
                        anno: [],
                        options: [
                            {
                                id: 11,
                                label: "是",
                                anno: [],
                            },
                            {
                                id: 12,
                                label: "否",
                                anno: [],
                            }
                        ],
                    },
                    {
                        id: 16,
                        name: "AESCONG",
                        label: "先天性异常/出生缺陷",
                        itemType: "selection",
                        anno: [],
                        options: [
                            {
                                id: 13,
                                label: "是",
                                anno: [],
                            },
                            {
                                id: 14,
                                label: "否",
                                anno: [],
                            }
                        ],
                    },
                ],
            },
            {
                id: 2,
                name: "DM",
                label: "人口统计学资料",
                group: [],
                anno: [],
                items: [
                    {
                        id: 17,
                        name: "BRTHDAT",
                        label: "出生日期",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 18,
                        name: "AGE",
                        label: "年龄",
                        itemType: "text",
                        unit: {
                            name: "岁",
                            anno: []
                        },
                        anno: [],
                        options: [],
                    },
                    {
                        id: 19,
                        name: "SEX",
                        label: "性别",
                        itemType: "selection",
                        anno: [],
                        options: [
                            {
                                id: 15,
                                label: "男性",
                                anno: [],
                            },
                            {
                                id: 16,
                                label: "女性",
                                anno: [],
                            }
                        ],
                    },
                    {
                        id: 20,
                        name: "FECUND",
                        label: "若为女性，是否有生育的可能性？",
                        itemType: "selection",
                        anno: [],
                        options: [
                            {
                                id: 17,
                                label: "是",
                                anno: [],
                            },
                            {
                                id: 18,
                                label: "否",
                                anno: [],
                            }
                        ],
                    },
                    {
                        id: 21,
                        name: "FECREAND",
                        label: "若否，请选择原因",
                        itemType: "selection",
                        anno: [],
                        options: [
                            {
                                id: 19,
                                label: "绝经",
                                anno: [],
                            },
                            {
                                id: 20,
                                label: "绝育",
                                anno: [],
                            },
                            {
                                id: 21,
                                label: "其他",
                                anno: [],
                            }
                        ],
                    },
                    {
                        id: 22,
                        name: "FECREAO",
                        label: "若为其他，请描述",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 23,
                        name: "ETHNIC",
                        label: "民族",
                        itemType: "selection",
                        anno: [],
                        options: [
                            {
                                id: 22,
                                label: "汉族",
                                anno: [],
                            },
                            {
                                id: 23,
                                label: "其他",
                                anno: [],
                            },
                        ],
                    },
                    {
                        id: 24,
                        name: "ETHNOTH",
                        label: "若为其他，请描述",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                ],
            },

            {
                id: 3,
                name: "EG",
                label: "12导联心电图",
                group: [],
                anno: [],
                items: [
                    {
                        id: 25,
                        name: "EGPERF",
                        label: "是否进行心电图检查？",
                        itemType: "selection",
                        anno: [],
                        options: [
                            {
                                id: 22,
                                label: "是",
                                anno: [],
                            },
                            {
                                id: 23,
                                label: "否",
                                anno: [],
                            }],
                    },
                    {
                        id: 26,
                        name: "EGREASND",
                        label: "若否，请说明原因",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 27,
                        name: "EGDAT",
                        label: "检查日期",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 28,
                        name: "EGTIM",
                        label: "检查时间",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 29,
                        name: "EGHR",
                        label: "心率",
                        itemType: "text",
                        anno: [],
                        unit: {
                            name: "次/分",
                            anno: []
                        },
                        options: [],
                    },
                    {
                        id: 30,
                        name: "EGRR",
                        label: "RR 间期",
                        itemType: "text",
                        anno: [],
                        unit: {
                            name: "msec",
                            anno: []
                        },
                        options: [],
                    },
                    {
                        id: 31,
                        name: "EGPR",
                        label: "PR 间期",
                        itemType: "text",
                        anno: [],
                        unit: {
                            name: "msec",
                            anno: []
                        },
                        options: [],
                    },
                    {
                        id: 32,
                        name: "EGQRS",
                        label: "QRS",
                        itemType: "text",
                        anno: [],
                        unit: {
                            name: "msec",
                            anno: []
                        },
                        options: [],
                    },
                    {
                        id: 33,
                        name: "EGQT",
                        label: "QT 间期",
                        itemType: "text",
                        anno: [],
                        unit: {
                            name: "msec",
                            anno: []
                        },
                        options: [],
                    },
                    {
                        id: 34,
                        name: "EGQTC",
                        label: "QTc",
                        itemType: "text",
                        anno: [],
                        unit: {
                            name: "msec",
                            anno: []
                        },
                        options: [],
                    },
                    {
                        id: 35,
                        name: "EGQTCMET",
                        label: "QTc计算方法",
                        itemType: "selection",
                        anno: [],
                        options: [
                            {
                                id: 24,
                                label: "Bazett公式",
                                anno: [],
                            },
                            {
                                id: 25,
                                label: "Fridericia公式",
                                anno: [],
                            },
                            {
                                id: 26,
                                label: "机器读取",
                                anno: [],
                            }
                        ],
                    },
                    {
                        id: 36,
                        name: "EGCLSIG",
                        label: "心电图总体结果评价",
                        itemType: "selection",
                        anno: [],
                        options: [
                            {
                                id: 27,
                                label: "正常",
                                anno: [],
                            },
                            {
                                id: 28,
                                label: "异常，无临床意义",
                                anno: [],
                            },
                            {
                                id: 29,
                                label: "异常，有临床意义",
                                anno: [],
                            }
                        ],
                    },
                    {
                        id: 37,
                        name: "EGSPEC",
                        label: "若异常，请说明",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                ],
            },

            {
                id: 4,
                name: "EC",
                label: "试验药物注射记录",
                group: [],
                anno: [],
                items: [
                    {
                        id: 38,
                        name: "ECYN",
                        label: "受试者本访视是否接受了试验药物给药？",
                        itemType: "selection",
                        anno: [],
                        options: [
                            {
                                id: 30,
                                label: "是",
                                anno: [],
                            },
                            {
                                id: 31,
                                label: "否",
                                anno: [],
                            }
                        ],
                    },
                    {
                        id: 39,
                        name: "ECREASND",
                        label: "未给药原因",
                        itemType: "selection",
                        anno: [],
                        options: [
                            {
                                id: 32,
                                label: "不良事件",
                                anno: [],
                            },
                            {
                                id: 33,
                                label: "突发公共卫生事件或其他不可抗力",
                                anno: [],
                            },
                            {
                                id: 34,
                                label: "其他",
                                anno: [],
                            }
                        ],
                    },
                    {
                        id: 40,
                        name: "ECAENO1",
                        label: "若未给药原因为不良事件，请选择不良事件编号",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 41,
                        name: "ECREAOTH",
                        label: "若为其他，请说明",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 42,
                        name: "ECLOT",
                        label: "药物编号",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 43,
                        name: "ECSTDAT",
                        label: "注射日期",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 44,
                        name: "ECSTTIM",
                        label: "注射开始时间",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 45,
                        name: "ECLOC",
                        label: "注射部位",
                        itemType: "selection",
                        anno: [],
                        options: [
                            {
                                id: 35,
                                label: "腹部",
                                anno: [],
                            },
                            {
                                id: 36,
                                label: "上臂",
                                anno: [],
                            },
                            {
                                id: 37,
                                label: "大腿",
                                anno: [],
                            },
                            {
                                id: 38,
                                label: "其他",
                                anno: [],
                            }
                        ],
                    },
                    {
                        id: 46,
                        name: "ECLOCSP",
                        label: "其他，请说明",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 47,
                        name: "ECCOMYN",
                        label: "是否完成药物注射？",
                        itemType: "selection",
                        anno: [],
                        options: [
                            {
                                id: 39,
                                label: "是",
                                anno: [],
                            },
                            {
                                id: 40,
                                label: "否",
                                anno: [],
                            },
                        ],
                    },
                    {
                        id: 48,
                        name: "ECNCREA",
                        label: "若否，请说明原因",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 49,
                        name: "ECAENO2",
                        label: "若为不良事件，请选择不良事件编号",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 50,
                        name: "ECNCREAO",
                        label: "若为其他，请说明",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                ],
            },


            {
                id: 5,
                name: "LBOTH",
                label: "其他实验室检查",
                group: [],
                anno: [],
                items: [
                    {
                        id: 51,
                        name: "LBTEST5",
                        label: "检查名称",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 52,
                        name: "LBDAT5",
                        label: "采样日期",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 53,
                        name: "LBORRES5",
                        label: "检查结果",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 54,
                        name: "LBUNIT5",
                        label: "单位",
                        itemType: "selection",
                        anno: [],
                        options: [
                            {
                                id: 41,
                                label: "%",
                                anno: [],
                            },
                            {
                                id: 42,
                                label: "10^6/L",
                                anno: [],
                            },
                            {
                                id: 43,
                                label: "10^9/L",
                                anno: [],
                            },
                            {
                                id: 44,
                                label: "cells/uL",
                                anno: [],
                            },
                            {
                                id: 45,
                                label: "IU/L",
                                anno: [],
                            },
                            {
                                id: 46,
                                label: "其他",
                                anno: [],
                            }
                        ],
                    },
                    {
                        id: 55,
                        name: "LBUNITO5",
                        label: "若为其他，请说明",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 56,
                        name: "LBRNRLO5",
                        label: "正常值范围下限",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 57,
                        name: "LBRNRHI5",
                        label: "正常值范围上限",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                    {
                        id: 58,
                        name: "LBCLSIG5",
                        label: "结果评估",
                        itemType: "selection",
                        anno: [],
                        options: [{
                            id: 47,
                            label: "正常",
                            anno: [],
                        },
                        {
                            id: 48,
                            label: "异常，无临床意义",
                            anno: [],
                        },
                        {
                            id: 49,
                            label: "异常，有临床意义",
                            anno: [],
                        }],
                    },
                    {
                        id: 59,
                        name: "LBDESC5",
                        label: "如异常有临床意义，请说明",
                        itemType: "text",
                        anno: [],
                        options: [],
                    },
                ],
            },
        ],
    };
}

function fakeId(): number {
    return (new Date()).valueOf();
}

export async function createDomain(formId: number, name: string, label: string) {
    const { project } = storeToRefs(useAtemStore());
    const form = project.value.forms.find(f => f.id === formId);
    if (form) {
        form.group.push({
            id: form.group.length,
            name,
            label,
        });
    }
}

export async function createFormAnno(formId: number, groupId: number, value: string, assign: boolean) {
    const { project } = storeToRefs(useAtemStore());
    let targetFormIndex = -1;
    for (const [formIndex, form] of project.value.forms.entries()) {
        if (form.id === formId) {
            targetFormIndex = formIndex;
            break;
        }
    }
    if (targetFormIndex > -1) {
        project.value.forms[targetFormIndex].anno.push({
            id: fakeId(),
            group: groupId,
            value,
            assign,
        });
    }
}

export async function createItemAnno(itemId: number, groupId: number, value: string, assign: boolean) {
    const { project } = storeToRefs(useAtemStore());
    let targetFormIndex = -1;
    let targetItemIndex = -1;

    for (const [formIndex, form] of project.value.forms.entries()) {
        for (const [itemIndex, item] of form.items.entries()) {
            if (item.id === itemId) {
                targetFormIndex = formIndex;
                targetItemIndex = itemIndex;
                break;
            }
        }
    }

    if (targetFormIndex > -1 && targetItemIndex > -1) {
        project.value.forms[targetFormIndex].items[targetItemIndex].anno.push({
            id: fakeId(),
            group: groupId,
            value,
            assign,
        });
    }
}

export async function createItemUnitAnno(itemId: number, groupId: number, value: string, assign: boolean) {
    const { project } = storeToRefs(useAtemStore());
    let targetFormIndex = -1;
    let targetItemIndex = -1;

    for (const [formIndex, form] of project.value.forms.entries()) {
        for (const [itemIndex, item] of form.items.entries()) {
            if (item.id === itemId) {
                targetFormIndex = formIndex;
                targetItemIndex = itemIndex;
                break;
            }
        }
    }

    if (targetFormIndex > -1 && targetItemIndex > -1) {
        project.value.forms[targetFormIndex].items[targetItemIndex].unit?.anno.push({
            id: fakeId(),
            group: groupId,
            value,
            assign,
        });
    }
}

export async function createOptionAnno(optionId: number, groupId: number, value: string, assign: boolean) {
    const { project } = storeToRefs(useAtemStore());
    let targetFormIndex = -1;
    let targetItemIndex = -1;
    let targetOptionIndex = -1;

    for (const [formIndex, form] of project.value.forms.entries()) {
        for (const [itemIndex, item] of form.items.entries()) {
            for (const [optionIndex, option] of item.options.entries()) {
                if (option.id === optionId) {
                    targetFormIndex = formIndex;
                    targetItemIndex = itemIndex;
                    targetOptionIndex = optionIndex;
                }
            }
        }
    }

    if (targetFormIndex > -1 && targetItemIndex > -1 && targetOptionIndex > -1) {
        project.value.forms[targetFormIndex].items[targetItemIndex].options[targetOptionIndex].anno.push({
            id: fakeId(),
            group: groupId,
            value,
            assign,
        });
    }
}

export async function removeAnno(id: number) {
    const { project } = storeToRefs(useAtemStore());
    for (const form of project.value.forms) {
        for (let [index, anno] of form.anno.entries()) {
            if (anno.id === id) {
                form.anno.splice(index, 1);
                return;
            }
        }
        for (const item of form.items) {
            for (let [index, anno] of item.anno.entries()) {
                if (anno.id === id) {
                    item.anno.splice(index, 1);
                    return;
                }
            }
            if (item.options) {
                for (const option of item.options) {
                    for (let [index, anno] of option.anno.entries()) {
                        console.log(anno);
                        if (anno.id === id) {
                            option.anno.splice(index, 1);
                            return;
                        }
                    }
                }
            }
        }
    }
}

export async function updateAnno(id: number, group: number, value: string, assign: boolean) {
    const { project } = storeToRefs(useAtemStore());
    for (const form of project.value.forms) {
        for (let [index, anno] of form.anno.entries()) {
            if (anno.id === id) {
                anno.value = value;
                anno.group = group;
                anno.assign = assign;
                return;
            }
        }
        for (const item of form.items) {
            for (let [index, anno] of item.anno.entries()) {
                if (anno.id === id) {
                    anno.value = value;
                    anno.group = group;
                    anno.assign = assign;
                    return;
                }
            }
            if (item.options) {
                for (const option of item.options) {
                    for (let [index, anno] of option.anno.entries()) {
                        if (anno.id === id) {
                            anno.value = value;
                            anno.group = group;
                            anno.assign = assign;
                            return;
                        }
                    }
                }
            }
        }
    }
}

export async function getForm(id: number): Promise<Form> {
    const { project } = storeToRefs(useAtemStore());
    let targetIndex = -1;
    for (const [index, form] of project.value.forms.entries()) {
        if (form.id === id) {
            targetIndex = index;
        }
    }
    return project.value.forms[targetIndex]
}

export async function sdtmDomains(): Promise<Group[]> {
    return [
        {
            id: 1,
            name: "AE",
            label: "不良事件",
        },
        {
            id: 2,
            name: "EC",
            label: "暴露采集",
        },
        {
            id: 3,
            name: "EG",
            label: "心电图",
        },
        {
            id: 4,
            name: "VS",
            label: "生命体征",
        },
        {
            id: 5,
            name: "PC",
            label: "药代动力学浓度",
        },
        {
            id: 6,
            name: "DM",
            label: "人口学",
        },
        {
            id: 7,
            name: "DS",
            label: "处置",
        },
        {
            id: 8,
            name: "PE",
            label: "体格检查",
        },
        {
            id: 9,
            name: "RP",
            label: "生殖系统检查",
        },
        {
            id: 10,
            name: "DC",
            label: "人口学采集",
        },
    ];
}

export async function llmAnnotation(): Promise<Project> {
    const result = await axios.get("/api/llm");
    return result.data
}

export enum AnnoLevel {
    Form,
    Item,
    Option,
    Unit,
}

export interface Project {
    id: number;
    name: string;
    forms: Form[];
}

export interface Form {
    id: number;
    name: string;
    label: string;
    group: Group[];
    anno: Anno[];
    items: Item[];
}

export interface Group {
    id: number;
    name: string;
    label: string;
}

export interface Anno {
    id: number;
    group: number;
    value: string;
    assign: boolean;
}

export interface Item {
    id: number;
    name: string;
    label: string;
    itemType: string;
    anno: Anno[];
    options: ItemOption[];
    unit?: Unit;
}

export interface Unit {
    name: string,
    anno: Anno[],
}

export interface ItemOption {
    id: number;
    label: string;
    anno: Anno[];
}