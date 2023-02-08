<template>
  <v-app theme="dark">
    <v-navigation-drawer v-model="drawer">
      <v-list>
        <v-list-item
          prepend-avatar="https://i.pravatar.cc/300?u=0"
          title="[未填名字]"
        ></v-list-item>
      </v-list>
      <v-divider></v-divider>

      <v-list density="compact" nav>
        <v-list-item :to="'/'" prepend-icon="mdi-home" title="首頁" value="homepage"></v-list-item>
        <v-list-item :to="'/books'" prepend-icon="mdi-folder" title="書籍" value="books"></v-list-item>
        <v-list-item :to="'/config'" prepend-icon="mdi-widgets" title="個人設定" value="config"></v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-app-bar app density="compact">
      <v-app-bar-nav-icon @click="drawer = !drawer"></v-app-bar-nav-icon>

      <v-toolbar-title> 我的圖書館 </v-toolbar-title>
    </v-app-bar>

    <v-main>
      <router-view></router-view>
    </v-main>
  </v-app>
</template>

<script>
  export default {
    data: () => ({ drawer: null }),
  }
</script>
