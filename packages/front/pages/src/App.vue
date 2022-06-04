<script setup lang="ts">
import { RouterView } from "vue-router";
import { ElMenu, ElMenuItem } from "element-plus";
import { ElButton } from "element-plus";
</script>

<template>
  <header>
    <ElMenu mode="horizontal" class="h-16 py-2" router>
      <div>
        <h1
          class="hover:text-blue-300 text-slate-600 dark:text-slate-100 text-2xl xs:text-lg py-1 pl-6 pr-8"
        >
          Magnifique
        </h1>
      </div>
      <ElMenuItem index="/login/">登录</ElMenuItem>

      <ElMenuItem index="/class/">班级</ElMenuItem>

      <ElMenuItem index="/member/">成员</ElMenuItem>

      <ElMenuItem index="/admin/">管理员</ElMenuItem>

      <div class="float-right text-right">
        <div>
          <ElButton text bg type="primary" circle class="float-right"
            >A</ElButton
          >
        </div>
      </div>
    </ElMenu>
  </header>

  <RouterView />
</template>
