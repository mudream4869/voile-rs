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
            <v-text-field type="text" label="使用者名字" v-model="user_config.name" append-icon="mdi-send"
              @click:append="updateUserConfig"></v-text-field>
            <v-file-input :rules="avatar_rules" accept="image/png, image/jpeg" label="使用者頭像" show-size
              @change="uploadAvatar($event)"></v-file-input>
          </v-form>
        </v-expansion-panel-text>
      </v-expansion-panel>
      <v-expansion-panel title="使用者偏好">
        <v-expansion-panel-text>
          <v-form class="ma-md-2">
            <v-select label="主題" :items="['light', 'dark']" @update:modelValue="updateUserConfig"
              v-model="user_config.theme"></v-select>
            <v-text-field label="小說字體大小"></v-text-field>
          </v-form>
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>
  </v-app>
</template>

<script>
import { uploadAvatar, updateUserConfig, getUserConfig } from '@/api/users';
import { useTheme } from 'vuetify'

export default {
  data: () => {
    return {
      setting_filename: '~/.voile/config',
      data_dir: '~/.voile/books',
      db_filename: '~/.voile/db.sqlite',

      avatar_rules: [
        value => {
          return !value || !value.length || value[0].size < 2000000 ||
            'Avatar size should be less than 2 MB!'
        },
      ],

      user_config: {
        name: '',
        theme: 'light',
      },
    }
  },
  methods: {
    uploadAvatar(event) {
      const avatar_file = event.target.files[0];
      uploadAvatar(avatar_file);
    },

    async updateUserConfig(value) {
      this.user_config.theme = value
      await updateUserConfig(this.user_config)
      this.theme.global.name.value = this.user_config.theme
    },

    async fetchUserConfig() {
      this.user_config = await getUserConfig()
      this.theme.global.name.value = this.user_config.theme
    },
  },
  setup() {
    const theme = useTheme()

    return {
      theme,
    }
  },
  created() {
    this.fetchUserConfig()
  },
}
</script>
