<template>
  <v-app>
    <v-form class="ma-md-2">
      <v-text-field label="設定檔案" v-model="setting_filename" disabled></v-text-field>
      <v-text-field label="本地儲存路徑" v-model="data_dir" disabled></v-text-field>
      <v-text-field label="資料庫儲存路徑" v-model="db_filename" disabled></v-text-field>
      <v-btn color="success" variant="elevated">儲存</v-btn>
    </v-form>
  </v-app>
</template>

<script>
  export default {
    data: () => {
      return {
        setting_filename: '~/.voile/config',
        data_dir: '~/.voile/books',
        db_filename: '~/.voile/db.sqlite',
      }
    },
  }
</script>
