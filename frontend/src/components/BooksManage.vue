<template>
  <v-app>
    <v-container fluid>
      <v-row>
        <v-col cols="3">
          <v-chip class="ma-2" v-for="btype in book_types" :key="btype" :color="book_type == btype ? 'red' : 'default'"
            @click="toggleBookType(btype)" label> {{ btype }} </v-chip>
        </v-col>

        <v-col cols="6">
          <v-chip class="ma-2" v-for="tag in tags" :key="tag" :color="used_tags.has(tag) ? 'green' : 'default'"
            @click="toggleTag(tag)" label> {{ tag }} </v-chip>
        </v-col>

        <v-col cols="3">
          <v-btn :disabled="selected_bookids.length == 0" v-on:click="delete_alert_show = true" variant="outlined"
            color="red"> 刪除 </v-btn>
          <v-btn :disabled="selected_bookids.length == 0" v-on:click="toggleChangeType()" variant="outlined" color="blue">
            修改類別 </v-btn>
        </v-col>

      </v-row>
      <v-dialog transition="dialog-top-transition" width="auto" v-model="delete_alert_show">
        <v-card>
          <v-card-title> 刪除 {{ selected_bookids.length }} 本書 </v-card-title>
          <v-card-text>
            確認刪除 {{ selected_bookids.length }} 本書？
            警告：此操作將無法被還原。
            <v-btn v-on:click="deleteBooks()" color="red"> 確認 </v-btn>
            <v-btn v-on:click="delete_alert_show = false"> 取消 </v-btn>
          </v-card-text>
        </v-card>
      </v-dialog>
      <v-dialog transition="dialog-top-transition" width="auto" v-model="change_type.show">
        <v-card>
          <v-card-title> 修改 {{ selected_bookids.length }} 本書的類別 </v-card-title>
          <v-card-text>
            <v-autocomplete label="分類" v-model="change_type.book_type" :items="book_types"></v-autocomplete>
            <v-btn v-on:click="changeBooksType()" color="blue"> 確認修改 </v-btn>
          </v-card-text>
        </v-card>
      </v-dialog>
      <v-dialog transition="dialog-top-transition" width="auto" v-model="detail.show">
        <v-card>
          <v-card-title> {{ detail.book.title }}</v-card-title>
          <v-card-text>
            <v-row>
              <v-col cols="3"> 標題 </v-col>
              <v-col cols="9"> {{ detail.book.title }} </v-col>
              <v-col cols="3"> 類別 </v-col>
              <v-col cols="9"> {{ detail.book.book_type || "<NULL>" }} </v-col>
              <v-col cols="3"> 作者 </v-col>
              <v-col cols="9"> {{ detail.book.author }} </v-col>
              <v-col cols="3"> 標籤 </v-col>
              <v-col cols="9"> {{ detail.book.tags }} </v-col>
              <v-col cols="3"> 新增日期 </v-col>
              <v-col cols="9"> {{ detail.book.created_time }} </v-col>
              <v-col cols="3"> 編輯日期 </v-col>
              <v-col cols="9"> {{ detail.book.modified_time }} </v-col>
              <v-col cols="3"> 網頁觀看位置 </v-col>
              <v-col cols="9">
                <v-btn :to="{ name: 'book', params: { book_id: detail.book.book_id } }">Go</v-btn>
              </v-col>
              <v-col cols="3"> 實際資料夾位置 </v-col>
              <v-col cols="9"> {{ detail.book.local_path }} </v-col>
              <v-col cols="3"> 實際檔案 </v-col>
              <v-col cols="9"> {{ detail.book.content_titles }} </v-col>
            </v-row>
          </v-card-text>
        </v-card>
      </v-dialog>
      <v-table>
        <thead>
          <tr>
            <th class="text-left">
              選擇
            </th>
            <th class="text-left">
              書籍類別
            </th>
            <th class="text-left">
              標題
            </th>
            <th class="text-left">
              作者
            </th>
            <th class="text-left">
              標籤
            </th>
            <th class="text-left">
              詳細資料
            </th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="book in show_books" :key="book.book_id">
            <td class="text-xs-center">
              <v-checkbox class="d-inline-flex" :value="book.book_id" v-model="selected_bookids">
              </v-checkbox>
            </td>
            <td>{{ book.book_type || '<NULL>' }}</td>
            <td>{{ book.title }}</td>
            <td>{{ book.author }}</td>
            <td>
              <v-chip class="ma-2" label v-for="tag in book.tags" :key="tag"> {{ tag }} </v-chip>
            </td>
            <td>
              <v-btn v-on:click="showBookDetail(book.book_id)">詳細資料</v-btn>
            </td>
          </tr>
        </tbody>
      </v-table>
    </v-container>
  </v-app>
</template>

<script>
import { deleteBook, getAllTags } from '@/api/books'
import { updateBookDetail } from '@/api/books'
import { getAllTypes } from '@/api/books'
import { getAllBooks } from '@/api/books'

export default {
  data: () => {
    return {
      selected_bookids: [],
      books: [],
      book_types: [],
      book_type: null,

      tags: [],
      used_tags: new Set(),

      detail: {
        show: false,
        book_id: null,
        book: {},
      },

      change_type: {
        show: false,
        book_type: '',
      },

      delete_alert_show: false,
    }
  },
  created() {
    // fetch on init
    this.fetchData()
  },
  computed: {
    show_books() {
      return this.books.filter(book => {
        var match = true
        this.used_tags.forEach(tag => {
          if (!book.tags_set.has(tag)) {
            match = false
          }
        })

        if (this.book_type == null) {
          // ok
        } else if (this.book_type == '<NULL>' && book.book_type == undefined) {
          // ok
        } else if (this.book_type == book.book_type) {
          // ok
        } else {
          match = false
        }

        return match
      })
    }
  },
  methods: {
    async fetchData() {
      this.books = await getAllBooks()
      this.tags = await getAllTags()
      this.book_types = await getAllTypes()
      this.book_types.push('<NULL>')
    },
    toggleTag(tag) {
      if (this.used_tags.has(tag)) {
        this.used_tags.delete(tag)
      } else {
        this.used_tags.add(tag)
      }
    },
    toggleBookType(btype) {
      if (this.book_type == btype) {
        this.book_type = null
      } else {
        this.book_type = btype
      }
    },
    toggleChangeType() {
      this.change_type.show = true
    },
    showBookDetail(book_id) {
      this.detail.show = true
      this.detail.book_id = book_id
      this.detail.book = this.books.find(book => book.book_id == book_id)
    },
    changeBooksType() {
      this.change_type.show = false
      if (this.selected_bookids.length == 0 || this.change_type.book_type == '') {
        return
      }
      this.selected_bookids.forEach(book_id => {
        updateBookDetail(book_id, {
          book_type: this.change_type.book_type,
        })
      })
      this.books.forEach(book => {
        if (this.selected_bookids.indexOf(book.book_id) != -1) {
          book.book_type = this.change_type.book_type
        }
      })

      this.selected_bookids = []
      this.change_type.book_type = ''
    },
    deleteBooks() {
      this.delete_alert_show = false;
      if (this.selected_bookids.length == 0) {
        return
      }

      this.selected_bookids.forEach(book_id => {
        deleteBook(book_id)
      })

      this.books = this.books.filter(book => {
        return (this.selected_bookids.indexOf(book.book_id) == -1);
      })

      this.selected_bookids = []
    }
  },
}
</script>
