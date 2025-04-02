<script lang="ts" setup>
import { createDomain, sdtmDomains, type Anno, type Group } from '@/api/project';
import { onMounted, ref, type Ref } from 'vue';

const { formId } = defineProps<{ formId: number }>();
const domains: Ref<Group[]> = ref([]);
const domain = ref({ name: '', label: '' });
const emit = defineEmits<{ (e: "close"): void }>();

function changeDomain(name: string) {
    domain.value.label = domains.value.find((d) => d.name === name)?.label || '';
}

async function create() {
    const { name, label } = domain.value;
    await createDomain(formId, name, label);
    emit('close');
}

function close() {
    emit('close');
}

onMounted(async () => {
    domains.value = await sdtmDomains();
});

</script>

<template>
    <el-form :model="domain" label-width="auto">
        <el-form-item label="Domain">
            <el-select filterable allow-create @change="changeDomain" v-model="domain.name">
                <el-option v-for="domain in domains" :key="domain.id" :label="domain.name" :value="domain.name">
                </el-option>
            </el-select>
        </el-form-item>
        <el-form-item label="Label">
            <el-input v-model="domain.label" />
        </el-form-item>
        <el-form-item>
            <el-button @click="create" type="primary" plain>
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

<style scoped>
.add-domain {
    margin-top: 5px;
    height: 20px;
    width: 100%;
}
</style>