<script lang="ts" setup>
import { removeAnno, updateAnno, type Anno, type Group } from '@/api/project';
import { colorTopic } from './styles';
import { ref } from 'vue';

const { annotation, groups } = defineProps<{ annotation: Anno, groups: Group[] }>();
const editDisplay = ref(false);

async function deleteAnno() {
    await removeAnno(annotation.id);
    editDisplay.value = false;
}

async function modifyAnno() {
    const { id, value, group, assign } = annotation;
    await updateAnno(id, group, value, assign);
    editDisplay.value = false;

}


</script>

<template>
    <el-tag style="margin-left: 3px;" :class="annotation.assign ? 'assign' : ''" :type="colorTopic(annotation.group)">
        {{ annotation.value }}
        <el-button @click="editDisplay = true" class="btn" :type="colorTopic(annotation.group)" size="small" text>
            <el-icon>
                <EditPen />
            </el-icon>
        </el-button>
    </el-tag>
    <el-dialog draggable destroy-on-close v-model="editDisplay" title="Update Annotation">
        <el-form :model="annotation" label-width="auto">
            <el-form-item label="Domain">
                <el-select v-model="annotation.group">
                    <el-option v-for="group in groups" :label="`${group.name}(${group.label})`" :value="group.id"
                        :key="group.id" />
                </el-select>
            </el-form-item>
            <el-form-item label="Value">
                <el-input v-model="annotation.value" clearable />
            </el-form-item>
            <el-form-item label="Assign">
                <el-switch v-model="annotation.assign" />
            </el-form-item>
            <el-form-item>
                <el-button @click="modifyAnno" plain type="primary">
                    <el-icon>
                        <Check />
                    </el-icon>
                </el-button>
                <el-button plain type="info">
                    <el-icon>
                        <Close />
                    </el-icon>
                </el-button>
                <el-button @click="deleteAnno" plain type="danger">
                    <el-icon>
                        <Delete />
                    </el-icon>
                </el-button>
            </el-form-item>
        </el-form>
    </el-dialog>
</template>

<style scoped>
.assign {
    border: 2px dotted;
}

.btn {
    width: 5px;
    height: 5px;
    margin: 0;
}
</style>