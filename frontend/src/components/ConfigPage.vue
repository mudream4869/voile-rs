<template>
  <v-app>
    <v-expansion-panels>
      <v-expansion-panel title="系統設定">
        <v-expansion-panel-text>
          這裡這設定只能在設定檔案裡面調整然後重啟，才能生效。
          <v-form class="ma-md-2">
            <v-text-field label="設定檔案" v-model="setting_filename" disabled></v-text-field>
            <v-text-field label="本地儲存路徑" v-model="data_dir" disabled></v-text-field>
            <v-text-field label="資料庫儲存路徑" v-model="db_filename" disabled></v-text-field>
          </v-form>
        </v-expansion-panel-text>
      </v-expansion-panel>
      <v-expansion-panel title="使用者設定">
        <v-expansion-panel-text>
          <v-form class="ma-md-2">
            <v-text-field label="使用者名字"></v-text-field>
            <v-file-input accept="image/*" label="使用者頭像"></v-file-input>
            <v-btn color="success" variant="elevated">儲存</v-btn>
          </v-form>
        </v-expansion-panel-text>
      </v-expansion-panel>
      <v-expansion-panel title="使用者偏好">
        <v-expansion-panel-text>
          <v-form class="ma-md-2">
            <v-text-field label="背景黑白"></v-text-field>
            <v-text-field label="小說字體大小"></v-text-field>
            <v-btn color="success" variant="elevated">儲存</v-btn>
          </v-form>
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>
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
