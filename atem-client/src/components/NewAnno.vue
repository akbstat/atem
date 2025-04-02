<script lang="ts" setup>
import { AnnoLevel, createFormAnno, createItemAnno, createItemUnitAnno, createOptionAnno, getForm, type Anno, type Group } from '@/api/project';
import { onMounted, ref, type Ref } from 'vue';
import { storeToRefs } from 'pinia';
import { useAtemStore } from '@/stores/counter';


const { project } = storeToRefs(useAtemStore());
const groups: Ref<Group[]> = ref([]);
const { formId, dest } = defineProps<{ formId: number, dest: { id: number, level: AnnoLevel } }>();
const emit = defineEmits<{ (e: "add", anno: Anno): void, (e: "close"): void }>();
const anno: Ref<Anno> = ref({
    id: (new Date()).valueOf(),
    group: 0,
    value: "",
    assign: false,
});

async function add() {
    const { group, value, assign } = anno.value;
    switch (dest.level) {
        case AnnoLevel.Form:
            await createFormAnno(dest.id, group, value, assign);
            break;
        case AnnoLevel.Item:
            await createItemAnno(dest.id, group, value, assign);
            break;
        case AnnoLevel.Option:
            await createOptionAnno(dest.id, group, value, assign);
            break;
        case AnnoLevel.Unit:
            await createItemUnitAnno(dest.id, group, value, assign);
            break;
        default:
    }
    close();
}

function close() {
    emit('close');
}

onMounted(async () => {
    groups.value = (await getForm(formId)).group;
})
</script>

<template>
    <el-form :model="anno" label-width="auto">
        <el-form-item label="Domain">
            <el-select v-model="anno.group">
                <el-option v-for="domain in groups" :id="domain.id" :label="domain.name" :value="domain.id"></el-option>
            </el-select>
        </el-form-item>
        <el-form-item label="Value">
            <el-input v-model="anno.value" />
        </el-form-item>
        <el-form-item label="Assign">
            <el-switch v-model="anno.assign" />
        </el-form-item>
        <el-form-item>
            <el-button @click="add" type="primary" plain>
                <el-icon>
                    <Check />
                </el-icon>
            </el-button>
            <el-button @click="close" type="danger" plain>
                <el-icon>
                    <Close />
                </el-icon>
            </el-button>
        </el-form-item>
    </el-form>
</template>

<style></style>