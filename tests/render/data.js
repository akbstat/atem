let data = {
    "project": "XX001-001",
    "forms": [
        {
            "name": "XX001 Administration",
            "group": [
                { id: 0, name: "EC", label: "Exposure as Collected" },
                { id: 1, name: "EX", label: "Exposure" },
            ],
            "anno": [
                { group: 0, value: "ECTRT = XX001" },
                { group: 1, value: "EXTRT = XX001" },
            ],
            "items": [
                {
                    "item_name": "EXYN",
                    "label": "Was XX001 administered?",
                    "item_type": "selection",
                    "anno": [
                        { group: 0, value: "ECOCCUR" },
                    ],
                    "selection": [
                        {
                            "label": "No",
                        },
                        {
                            "label": "Yes",
                        }
                    ]
                },
                {
                    "item_name": "EXREASND",
                    "label": "Reason not administered",
                    "item_type": "selection",
                    "anno": [
                        { group: 0, value: "ECRESOC in SUPPEC " },
                    ],
                    "selection": [
                        {
                            "label": "Adverse Event",
                        },
                        {
                            "label": "Other",
                        }
                    ]
                },
                {
                    "item_name": "EXAENO",
                    "label": "AE Identifier - Reason not administered",
                    "item_type": "text",
                    "anno": [
                        { group: 0, value: "ECADAEID in SUPPEC" },
                        { group: 0, value: "RELREC when ECADAEID = AESPID", assign: true },
                    ]
                },
                {
                    "item_name": "EXDOSE",
                    "label": "Actual Dose",
                    "item_type": "text",
                    "anno": [
                        { group: 0, value: "ECDOSE" },
                        { group: 1, value: "EXDOSE" },
                    ],
                    "unit": {
                        "name": "mg",
                        "anno": [
                            { group: 0, value: "ECDOSU" },
                            { group: 1, value: "EXDOSU" },
                        ]
                    }
                },
            ],
        },
        {
            "name": "血常规",
            "group": [
                { id: 0, name: "LB", label: "实验室检查" },
            ],
            "anno": [
                { group: 0, value: "LBCAT = 血常规", assign: true },
            ],
            "items": [
                {
                    "item_name": "LBPERF1",
                    "label": "是否进行血常规检查？",
                    "item_type": "selection",
                    "anno": [],
                    "selection": [
                        {
                            "label": "No",
                            "anno": [
                                { group: 0, value: "[NOT SUBMITTED]", assign: true },
                            ]
                        },
                        {
                            "label": "Yes",
                            "anno": [
                                { group: 0, value: "LBSTAT = 未查 when LBTESTCD = LBALL" },
                            ]
                        }
                    ]
                },
                {
                    "item_name": "LBREASND",
                    "label": "若否，请说明原因",
                    "item_type": "text",
                    "anno": [
                        { group: 0, value: "LBREASND" },
                    ],
                },
                {
                    "item_name": "LBDAT",
                    "label": "采样日期",
                    "item_type": "text",
                    "anno": [
                        { group: 0, value: "LBDTC" },
                    ]
                },
                {
                    "item_name": "LBTEST",
                    "label": "检查项目",
                    "item_type": "selection",
                    "anno": [],
                    "selection": [
                        {
                            "label": "红细胞计数(RBC)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = WBC" },
                            ]
                        },
                        {
                            "label": "血红蛋白(Hb)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = NEUT" },
                            ]
                        },
                        {
                            "label": "红细胞压积(HCT)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = EOS" },
                            ]
                        },
                        {
                            "label": "白细胞计数(WBC)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = BASO" },
                            ]
                        },
                        {
                            "label": "中性粒细胞计数(NEUT)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = LYM" },
                            ]
                        },
                        {
                            "label": "淋巴细胞计数(LYM)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = RBC" },
                            ]
                        },
                        {
                            "label": "嗜酸性粒细胞计数(EOS)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = HGB" },
                            ]
                        },
                        {
                            "label": "嗜碱性粒细胞计数(BASO)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = PLAT" },
                            ]
                        },
                        {
                            "label": "血小板计数(PLT)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = HCT" },
                            ]
                        },
                        {
                            "label": "天冬氨酸氨基转移酶(AST)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = PH" },
                            ]
                        },
                        {
                            "label": "丙氨酸氨基转移酶(ALT)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = UWBC" },
                            ]
                        },
                        {
                            "label": "碱性磷酸酶(ALP)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = UPROT" },
                            ]
                        },
                        {
                            "label": "乳酸脱氢酶(LDH)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = UGLUC" },
                            ]
                        },
                        {
                            "label": "总胆红素(TBil)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = KETONES" },
                            ]
                        },
                        {
                            "label": "直接胆红素(DBil)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = SPGRAV" },
                            ]
                        },
                        {
                            "label": "谷氨酰转移酶(GGT)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = OCCBLD" },
                            ]
                        },
                        {
                            "label": "总蛋白(TP)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = UWBCQ" },
                            ]
                        },
                        {
                            "label": "白蛋白(ALB)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = URBCQ" },
                            ]
                        },
                        {
                            "label": "血葡萄糖(GLU)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = ALT" },
                            ]
                        },
                        {
                            "label": "甘油三酯(TG)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = AST" },
                            ]
                        },
                        {
                            "label": "总胆固醇(TC)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = ALP" },
                            ]
                        },
                        {
                            "label": "高密度脂蛋白(HDL)",
                            "anno": [
                                { group: 0, value: "LBORRES when LBTESTCD = GGT" },
                            ]
                        },
                    ]
                },
                {
                    "item_name": "LBORRES",
                    "label": "检查结果",
                    "item_type": "text",
                    "anno": [
                        { group: 0, value: "LBORRES" },
                    ],
                },
                {
                    "item_name": "LBCLSIG1",
                    "label": "结果评估",
                    "item_type": "selection",
                    "anno": [],
                    "selection": [
                        {
                            "label": "正常",
                            "anno": [
                                { group: 0, value: "LBRESCAT = 正常" },
                            ]
                        },
                        {
                            "label": "异常，无临床意义",
                            "anno": [
                                { group: 0, value: "LBRESCAT = 异常" },
                                { group: 0, value: "LBCLSIG = N in SUPPLB" },
                            ]
                        },
                        {
                            "label": "异常，有临床意义",
                            "anno": [
                                { group: 0, value: "LBRESCAT = 异常" },
                                { group: 0, value: "LBCLSIG = Y in SUPPLB" },
                            ]
                        },
                        {
                            "label": "未做",
                            "anno": [
                                { group: 0, value: "LBDESC in SUPPLB" },
                            ]
                        }
                    ]
                },
                {
                    "item_name": "LBDESC",
                    "label": "临床意义注释",
                    "item_type": "text",
                    "anno": [
                        { group: 0, value: "LBDESC in SUPPLB" },
                    ],
                },
            ],
        }
    ]
};

