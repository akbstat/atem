<script lang="ts" setup>
import { AnnoLevel, getForm, getProject, llmAnnotation, removeAnno, type Anno, type Form, type Project } from '@/api/project';
import { onMounted, ref, type Ref, nextTick } from 'vue';
import NewDomain from './NewDomain.vue';
import NewAnno from './NewAnno.vue';
import { useAtemStore } from "@/stores/counter";
import { colorTopic } from './styles';
import { storeToRefs } from 'pinia';
import axios from "axios";
import { Cpu } from '@element-plus/icons-vue';


const { project } = storeToRefs(useAtemStore());
const form: Ref<Form> = ref({ id: 0, name: '', label: '', group: [], anno: [], items: [] });
const display = ref(false);

const currentItem = ref(0);
const currentFormId = ref(1);
const currentOption = ref(0);
const newDomainDisplay = ref(false);
const newAnnoDisplay = ref(false);
const newItemAnnoDisplay = ref(false);
const newOptionAnnoDisplay = ref(false);
const newItemUnitAnnoDisplay = ref(false);
const loading = ref(false);

function selectionClear() {
    currentItem.value = 0;
    currentOption.value = 0;
}

async function switchForm(id: number) {
    currentFormId.value = id;
    form.value = await getForm(id);
}

function showNewDomain() {
    newDomainDisplay.value = true;
}

function showNewAnno() {
    newAnnoDisplay.value = true;
}

function showNewItemAnno(id: number) {
    currentItem.value = id;
    newItemAnnoDisplay.value = true;
}

function showNewItemUnitAnno(id: number) {
    currentItem.value = id;
    newItemUnitAnnoDisplay.value = true;
}

function showNewOptionAnno(itemId: number, optionId: number) {
    currentOption.value = optionId;
    newOptionAnnoDisplay.value = true;
}

async function open() {
    await axios.post("/api", project.value);
    window.open(`http://localhost:3000`)
}

async function autoAnnotation() {
    loading.value = true;
    project.value = await llmAnnotation();
    form.value = await getForm(currentFormId.value);
    loading.value = false;
}

onMounted(async () => {
    project.value = await getProject();
    form.value = await getForm(1);
    display.value = true;
});

</script>

<template>
    <el-container v-if="display">
        <el-main v-loading="loading" element-loading-text="LLM Auto Annotating...">
            <el-container>
                <el-header>
                    <el-container>
                        <el-aside>
                            <span class="logo">A.T.E.M</span>
                        </el-aside>
                        <el-main>
                            <div style="display: flex;">
                                <el-breadcrumb style="margin-top: 9px;">
                                    <el-breadcrumb-item>{{ project.name }}</el-breadcrumb-item>
                                    <el-breadcrumb-item>{{ form.label }}({{ form.name }})</el-breadcrumb-item>
                                </el-breadcrumb>
                                <el-button @click="open" type="primary" text>
                                    <el-icon>
                                        <Printer />
                                    </el-icon>
                                </el-button>
                                <el-button style="margin: 0 0 0 0 ;" @click="autoAnnotation" type="primary" text>
                                    <el-icon>
                                        <Cpu />
                                    </el-icon>
                                </el-button>
                            </div>
                        </el-main>
                    </el-container>
                </el-header>
                <el-main>
                    <el-container>
                        <el-aside>
                            <el-menu>
                                <el-menu-item @click="() => { switchForm(form.id) }" v-for="form in project.forms"
                                    :key="form.name">
                                    <el-tag style="margin-right: 5px;" class="item-name" type="primary">{{ form.name
                                        }}</el-tag>
                                    {{ form.label }}
                                </el-menu-item>
                            </el-menu>
                        </el-aside>
                        <el-main>
                            <el-container>
                                <el-header style="height: auto;">
                                    <div class="form-header">
                                        <span style="padding-top: 5.5px; margin-right: 3px;">{{
                                            form.label }}({{
                                                form.name }})</span>
                                        <el-tag closable :type="colorTopic(domain.id)" style="margin: 3px 3px 0 0;"
                                            v-for="domain in form.group">{{
                                                `${domain.name}(${domain.label})` }}</el-tag>
                                        <el-button @click="showNewDomain" type="danger" text>
                                            <el-icon>
                                                <Plus />
                                            </el-icon>
                                        </el-button>
                                    </div>
                                    <el-card v-if="form.anno.length > 0">
                                        <div style="display: flex;">
                                            <Annotation v-for="anno in form.anno" :annotation="anno"
                                                :groups="form.group" />
                                            <el-button size="small" @click="showNewAnno" type="danger" text>
                                                <el-icon>
                                                    <Plus />
                                                </el-icon>
                                            </el-button>
                                        </div>
                                    </el-card>
                                    <el-button v-else @click="showNewAnno" type="danger" text>
                                        <el-icon>
                                            <Plus />
                                        </el-icon>
                                    </el-button>
                                </el-header>
                                <el-main>

                                    <el-scrollbar height="740px">
                                        <div v-for="item in form.items">
                                            <el-card v-if="item.itemType === 'selection'" class="item-card">
                                                <template #header>
                                                    <el-button @click="() => { showNewItemAnno(item.id) }"
                                                        type="primary" text>
                                                        <el-icon>
                                                            <Plus />
                                                        </el-icon>
                                                    </el-button>
                                                    <el-tag class="item-name" type="info">{{ item.name }}</el-tag>
                                                    <el-text style="padding: 3px 0 0 3px;"> {{ item.label }}</el-text>
                                                    <Annotation v-for="anno in item.anno" :annotation="anno"
                                                        :groups="form.group" />
                                                </template>
                                                <template #default>
                                                    <div v-for="option in item.options">
                                                        <el-button
                                                            @click="() => { showNewOptionAnno(item.id, option.id) }"
                                                            type="warning" text>
                                                            <el-icon>
                                                                <Plus />
                                                            </el-icon>
                                                        </el-button>
                                                        <el-text style="padding: 3px 0 0 3px;">{{ option.label
                                                        }}</el-text>
                                                        <Annotation v-for="anno in option.anno" :annotation="anno"
                                                            :groups="form.group" />
                                                    </div>
                                                </template>
                                            </el-card>
                                            <el-card v-else-if="item.itemType === 'checkbox'" class="item-card">
                                                <el-button @click="() => { showNewItemAnno(item.id) }" type="warning"
                                                    text>
                                                    <el-icon>
                                                        <Plus />
                                                    </el-icon>
                                                </el-button>
                                                <el-tag class="item-name" type="info">{{ item.name }}</el-tag>
                                                <el-text style="padding: 3px 0 0 3px;"> {{ item.label }}</el-text>
                                                <Annotation v-for="anno in item.anno" :annotation="anno"
                                                    :groups="form.group" />
                                            </el-card>
                                            <el-card v-else-if="item.itemType === 'label'" class="item-card">
                                                <el-text style="padding: 3px 0 0 3px;"> {{ item.label }}</el-text>
                                                <Annotation v-for="anno in item.anno" :annotation="anno"
                                                    :groups="form.group" />
                                            </el-card>
                                            <el-card v-else class="item-card">
                                                <el-button @click="() => { showNewItemAnno(item.id) }" type="primary"
                                                    text>
                                                    <el-icon>
                                                        <Plus />
                                                    </el-icon>
                                                </el-button>
                                                <el-tag class="item-name" type="info">{{ item.name }}</el-tag>
                                                <el-text style="padding: 3px 0 0 3px;"> {{ item.label }}</el-text>
                                                <Annotation v-for="anno in item.anno" :annotation="anno"
                                                    :groups="form.group" />
                                                <div v-if="item.unit">
                                                    <el-button @click="() => { showNewItemUnitAnno(item.id) }"
                                                        type="warning" text>
                                                        <el-icon>
                                                            <Plus />
                                                        </el-icon>
                                                    </el-button>
                                                    <el-tag type="info" round>{{ item.unit.name }}</el-tag>
                                                    <Annotation v-for="anno in item.unit.anno" :annotation="anno"
                                                        :groups="form.group" />
                                                    <!-- <el-text style="padding: 3px 0 0 3px;"> </el-text> -->
                                                </div>
                                            </el-card>
                                        </div>
                                    </el-scrollbar>

                                </el-main>
                            </el-container>
                        </el-main>
                    </el-container>
                </el-main>
            </el-container>
        </el-main>
    </el-container>
    <el-dialog draggable title="New Domain" width="50%" v-model="newDomainDisplay" destroy-on-close>
        <NewDomain :form-id="form.id" @close="() => { newDomainDisplay = false }" />
    </el-dialog>
    <el-dialog draggable v-model="newAnnoDisplay" title="New Form Annotation" destroy-on-close>
        <NewAnno :form-id="form.id" :dest="{ level: AnnoLevel.Form, id: form.id }"
            @close="() => { newAnnoDisplay = false }" />
    </el-dialog>
    <el-dialog draggable v-model="newItemAnnoDisplay" title="New Item Annotation" destroy-on-close>
        <NewAnno :form-id="form.id" :dest="{ level: AnnoLevel.Item, id: currentItem }"
            @close="() => { newItemAnnoDisplay = false }" />
    </el-dialog>
    <el-dialog draggable v-model="newOptionAnnoDisplay" title="New Option Annotation" destroy-on-close>
        <NewAnno :form-id="form.id" :dest="{ level: AnnoLevel.Option, id: currentOption }"
            @close="() => { newOptionAnnoDisplay = false }" />
    </el-dialog>
    <el-dialog draggable v-model="newItemUnitAnnoDisplay" title="New Item Unit Annotation" destroy-on-close>
        <NewAnno :form-id="form.id" :dest="{ level: AnnoLevel.Unit, id: currentItem }"
            @close="() => { newItemUnitAnnoDisplay = false }" />
    </el-dialog>
</template>

<style scoped>
.logo {
    margin-left: 6%;
    text-align: center;
    font-size: 40px;
}

.item-name {
    width: 80px;
}

.form-header {
    display: flex;
}

.item-card {
    margin-bottom: 5px;
}
</style>