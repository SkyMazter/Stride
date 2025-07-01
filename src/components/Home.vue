<template>
    <div class="home">
        <h3>Dashboard</h3>

        <div v-if="!is_loading && projects">
            <div v-for="project in projects" :key="project.id">
                <h3>{{ project.title }}</h3>
                <ul>
                    <li v-for="task in project.tasks" :key="task.id">
                        <label>
                            <input
                                type="checkbox"
                                v-model="checkedStates[project.id][task.id]"
                                @change="() => sendUpdate(project.id, task.id)"
                            />
                            {{ task.title }}
                        </label>
                    </li>
                </ul>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";

interface Task {
    id: number;
    title: string;
    is_completed: boolean;
}

interface Project {
    id: number;
    title: string;
    is_completed: boolean;
    tasks: Task[];
    percentage: number;
}

type TaskCheckboxMap = Record<number, Record<number, boolean>>;
//                ^ project_id    ^ task_id       ^ checked

const checkedStates = ref<TaskCheckboxMap>({});

const projects = ref<Project[] | null>(null);
const is_loading = ref<boolean>(false);

const loadData = async () => {
    is_loading.value = true;
    projects.value = null;

    try {
        const response = await invoke<Project[]>("get_all_projects");
        projects.value = response;

        checkedStates.value = Object.fromEntries(
            response.map((project) => [
                project.id,
                Object.fromEntries(
                    project.tasks.map((task) => [
                        task.id,
                        task.is_completed ?? false,
                    ]),
                ),
            ]),
        );
    } catch (error: any) {
        console.error("Error loading projects:", error);
    } finally {
        is_loading.value = false;
    }
};

const sendUpdate = async (project_id: number, task_id: number) => {
    try {
        const is_checked = checkedStates.value[project_id][task_id];

        const payload = {
            projectId: project_id,
            taskId: task_id,
            isChecked: is_checked,
        };

        console.log("📤 Sending to Rust:", payload);

        await invoke("update", payload);
    } catch (err) {
        console.error("❌ Failed to update task state:", err);
    }
};

onMounted(async () => loadData());
</script>
