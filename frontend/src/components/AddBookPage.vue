<template>
  <v-app>
    <v-form class="ma-md-2">
      <v-file-input
        accept="text/*, application/zip"
        :clearable="true"
        label="txt檔案/zip檔案"
        :key="fileInputKey"
        @change="uploadBook($event)"></v-file-input>
    </v-form>
    <div class="ma-md-2">
      <v-alert color="success" closable text="上傳成功" v-model="alertSuccess"></v-alert>
      <v-alert color="warning" closable text="上傳失敗" v-model="alertFail"></v-alert>
      <h2> 可上傳檔案格式 </h2>
      <h3> 單個 txt 檔案 </h3>
      <p> 通常會是小說類型 </p>
      <h3> 單個 zip 檔案 </h3>
      <p> 通常會是漫畫類型 </p>
    </div>
  </v-app>
</template>

<script>
export default {
  data: () => {
    return {
      fileInputKey: 0,
      alertFail: false,
      alertSuccess: false,
    }
  },
  methods: {
    uploadBook(event) {
      if (!event.target.files) {
        return
      }
      const avatar_file = event.target.files[0];
      const formData = new FormData();
      formData.append('book', avatar_file, avatar_file.name);
      fetch(`/api/books`, {
        method: 'POST',
        body: formData,
      }).then(res => {
        this.fileInputKey++
        this.alertSuccess = res.status == 200
        this.alertFail = !this.alertSuccess
      })
    },
  }
}
</script>
