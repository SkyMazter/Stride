<template>
    <div class="home">
        <nav>
            <h2>Dashboard</h2>
            <span>
                <button class="btn-primary" @click="handleClick">
                    Add New +
                </button>
            </span>
        </nav>

        <h3>Current Projects</h3>
        <div class="project-view" v-if="!is_loading && projects">
            <div
                class="project-card"
                v-for="project in projects"
                :key="project.id"
            >
                <h4>{{ project.title }}</h4>

                <span v-for="task in project.tasks" :key="task.id">
                    <label>
                        <input
                            type="checkbox"
                            v-model="checkedStates[project.id][task.id]"
                            @change="() => sendUpdate(project.id, task.id)"
                        />
                        {{ task.title }}
                    </label>
                </span>
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

<style scoped>
.home {
    width: 100%;
}
nav {
    display: flex;
    border-bottom: solid var(--greyscale-800) 0.5px;
    width: 100%;
    align-items: center;
    justify-content: space-between;
}
.project-view {
    display: flex;
    flex-flow: row wrap;
    gap: 1rem;
    justify-content: start;
    padding: 0.5rem;
}
.project-card {
    background-color: var(--greyscale-300);
    border-radius: 0.5rem;
    width: 45%;
    padding: 0.5rem;
}
h4 {
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
}
</style>
