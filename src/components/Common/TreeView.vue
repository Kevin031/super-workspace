<script setup lang="ts">
import { ref, computed } from 'vue';

export interface TreeNodeData {
  key: string;
  label: string;
  isProject?: boolean;
  children?: TreeNodeData[];
}

const props = defineProps<{
  node: TreeNodeData;
  selectedKey?: string;
}>();

const emit = defineEmits<{
  select: [key: string];
}>();

defineOptions({ name: 'TreeView' });

defineSlots<{
  prefix(props: { option: TreeNodeData }): any;
}>();

const isExpanded = ref(true);
const hasChildren = computed(() => !!props.node.children && props.node.children.length > 0);
const isSelected = computed(() => props.selectedKey === props.node.key);

function toggle(e: MouseEvent) {
  e.stopPropagation();
  isExpanded.value = !isExpanded.value;
}

function select() {
  emit('select', props.node.key);
}
</script>

<template>
  <div class="tree-node">
    <div
      class="tree-node-content"
      :class="{ 'tree-node-selected': isSelected }"
      @click="select"
    >
      <span
        v-if="hasChildren"
        class="tree-node-switcher"
        :class="{ 'tree-node-switcher-expanded': isExpanded }"
        @click="toggle"
      >
        <svg width="10" height="10" viewBox="0 0 24 24" fill="none">
          <path d="M9 18l6-6-6-6" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </span>
      <span v-else class="tree-node-switcher tree-node-switcher-placeholder" />
      <span class="tree-node-prefix">
        <slot name="prefix" :option="node" />
      </span>
      <span class="tree-node-label">{{ node.label }}</span>
    </div>
    <div v-if="hasChildren && isExpanded" class="tree-node-children">
      <TreeView
        v-for="child in node.children"
        :key="child.key"
        :node="child"
        :selected-key="selectedKey"
        @select="emit('select', $event)"
      >
        <template #prefix="slotProps">
          <slot name="prefix" :option="slotProps.option" />
        </template>
      </TreeView>
    </div>
  </div>
</template>

<style scoped>
.tree-node {
  color: var(--text-secondary);
  font-family: var(--font-body);
}

.tree-node-content {
  display: flex;
  align-items: center;
  gap: 4px;
  border-radius: var(--radius-sm);
  transition: all var(--transition-fast);
  padding: 5px 8px;
  font-size: 13.5px;
  font-weight: 450;
  line-height: 1.5;
  cursor: pointer;
  margin-bottom: 1px;
  position: relative;
}

.tree-node-content::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 0;
  background: var(--accent-primary);
  border-radius: 0 2px 2px 0;
  transition: height var(--transition-fast);
}

.tree-node-content:hover {
  background: var(--bg-hover);
}

.tree-node-selected {
  background: var(--accent-muted);
  transform: translateX(2px);
}

.tree-node-selected::before {
  height: 60%;
}

.tree-node-selected .tree-node-label {
  color: var(--accent-primary);
  font-weight: 600;
}

.tree-node-switcher {
  width: 18px;
  height: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  transition: all var(--transition-fast);
  cursor: pointer;
  flex-shrink: 0;
}

.tree-node-switcher:hover {
  color: var(--text-secondary);
}

.tree-node-switcher-expanded svg {
  transform: rotate(90deg);
}

.tree-node-switcher-placeholder {
  opacity: 0;
  pointer-events: none;
}

.tree-node-prefix {
  display: flex;
  align-items: center;
  color: var(--text-muted);
  font-size: 15px;
  transition: color var(--transition-fast);
  flex-shrink: 0;
}

.tree-node-content:hover .tree-node-prefix {
  color: var(--text-secondary);
}

.tree-node-selected .tree-node-prefix {
  color: var(--accent-primary);
}

.tree-node-label {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.tree-node-children {
  padding-left: 18px;
}
</style>
