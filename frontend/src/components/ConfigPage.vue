<template>
  <v-app>
    <v-expansion-panels>
      <v-expansion-panel title="啟動參數">
        <v-expansion-panel-text>
          <v-form class="ma-md-2">
            <v-text-field label="資料夾路徑" v-model="config_dir" disabled></v-text-field>
          </v-form>
        </v-expansion-panel-text>
      </v-expansion-panel>
      <v-expansion-panel title="系統設定">
        <v-expansion-panel-text>
          <v-form class="ma-md-2">
            <v-text-field label="監聽 IP" v-model="system_config.ip" disabled></v-text-field>
          </v-form>
          <v-form class="ma-md-2">
            <v-text-field label="監聽 Port" v-model="system_config.port" disabled></v-text-field>
          </v-form>
          <v-form class="ma-md-2">
            <v-text-field label="書籍資料夾" v-model="system_config.data_dir" disabled></v-text-field>
          </v-form>
          <v-form class="ma-md-2">
            <v-text-field label="伺服器資料夾" v-model="system_config.server_data_dir" disabled></v-text-field>
          </v-form>
        </v-expansion-panel-text>
      </v-expansion-panel>
      <v-expansion-panel title="使用者設定">
        <v-expansion-panel-text>
          <v-form class="ma-md-2">
            <v-text-field type="text" label="使用者名字" v-model="user_config.name" append-icon="mdi-send"
              @click:append="updateUserName"></v-text-field>
            <v-file-input :rules="avatar_rules" accept="image/png, image/jpeg" label="使用者頭像" show-size
              @change="uploadAvatar($event)"></v-file-input>
          </v-form>
        </v-expansion-panel-text>
      </v-expansion-panel>
      <v-expansion-panel title="使用者偏好">
        <v-expansion-panel-text>
          <v-form class="ma-md-2">
            <v-select label="主題" :items="['light', 'dark']" @update:modelValue="updateUserTheme"
              v-model="user_config.theme"></v-select>
          </v-form>
        </v-expansion-panel-text>
      </v-expansion-panel>
      <v-expansion-panel title="其他操作">
        <v-expansion-panel-text>
          <v-form class="ma-md-2">
            <v-btn @click="logout">登出</v-btn>
          </v-form>
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>
  </v-app>
</template>

<script>
import { uploadAvatar, updateUserConfig, getUserConfig, getSystemConfig } from '@/api/config';
import { useTheme } from 'vuetify'
import Cookies from 'js-cookie'

export default {
  data: () => {
    return {
      config_dir: '~/.voile/config',

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

      system_config: {
        ip: '127.0.0.1',
        port: 8080,
        data_dir: '',
        server_data_dir: '',
      },
    }
  },
  methods: {
    uploadAvatar(event) {
      const avatar_file = event.target.files[0];
      uploadAvatar(avatar_file);
    },

    logout() {
      Cookies.remove('has_login')
      this.$router.push({ name: 'login' })
    },

    async updateUserName() {
      await updateUserConfig({ name: this.user_config.name })
    },

    async updateUserTheme(value) {
      await updateUserConfig({ theme: value })
      this.theme.global.name.value = value
    },

    async fetchConfig() {
      this.user_config = await getUserConfig()
      this.system_config = await getSystemConfig()
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
    this.fetchConfig()
  },
}
</script>
