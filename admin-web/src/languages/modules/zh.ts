export default {
  project: {
    title: "海棠 CMS"
  },
  menu: {
    login: {
      auth: "登录",
      title: "海棠 CMS 管理平台",
      welcome: "欢迎登录",
      platform: "管理平台",
      description: "结构化优雅的内容管理体验",
      account: "账号密码登录",
      in: "登录",
      loading: "登录中",
      beiAnHao: "网站备案号",
      picture: "看不清，换一张",
      form: {
        loginName: "请输入用户名",
        password: "请输入密码",
        securityCode: "请输入验证码"
      },
      rules: {
        loginName: {
          required: "用户名不能为空",
          validator: "账号只能包含数字和字母"
        },
        password: {
          required: "密码不能为空",
          validator1: "长度在 6 到 20 个字符",
          validator2: "密码必须包含数字和字母",
        },
        securityCode: {
          required: "验证码不能为空"
        }
      }
    },
    home: {
      auth: "主控台",
      welcome: "欢迎使用海棠 CMS",
      welcomeBack: "欢迎回来",
      greetingFallback: "您好，愿您今天一切顺利！",
      defaultUser: "管理员",
      subtitle: "内容管理平台 — 高效、优雅、可靠",
      quickStart: "快捷入口",
      quickStartDesc: "常用功能一键直达",
      aboutTitle: "关于系统",
      aboutDesc: "{name} 是基于 Rust + Vue 3 的内容管理系统，公开站点由 Tera 模板驱动，管理后台独立构建。业务模块迁移完成后，数据概览将在此展示实时统计。",
      statPending: "统计数据待接入",
      linkPosts: "发布与管理文章内容",
      linkBanners: "配置首页与频道 Banner",
      linkMenus: "维护站点导航结构",
      linkUsers: "管理系统用户与权限",
      work: {
        name: "工作台"
      },
      analysis: {
        name: "分析页"
      },
      console: {
        name: "控制台"
      }
    },
    content: {
      auth: "内容管理",
      posts: "文章管理",
      categories: "分类管理",
      post: {
        manage: {
          title: "文章管理",
          subtitle: "创建与维护多语言文章内容",
          previewLang: "预览语言",
          titleCol: "标题",
          category: "分类",
          categoryPh: "请选择分类",
          status: "状态",
          draft: "草稿",
          published: "已发布",
          displayTime: "展示时间",
          displayTimePh: "选择展示时间",
          routePath: "路径",
          publicUrl: "公开链接",
          publicUrlHint: "当前站点实际访问地址；新建文章保存后才会生成 ID",
          publicUrlPending: "保存后生成",
          openPublic: "新窗口打开",
          seoPath: "SEO 路径（可选）",
          seoPathPh: "my-slug",
          seoPathHint: "仅填写 slug 段，前缀已固定；留空不影响访问，slug 路由尚未启用",
          seoPathSlugError: "slug 不能为空，且不能包含空格、# 或 ?",
          tags: "标签",
          tagsPh: "输入后按 Enter 添加，各语言可填本地化标签",
          description: "摘要",
          descriptionPh: "文章摘要",
          content: "正文",
          contentPh: "支持 Markdown 语法，可使用预览与全屏编辑",
          sectionMeta: "基本信息",
          sectionI18n: "多语言文案",
          create: "新建文章",
          edit: "编辑文章",
          titlePh: "文章标题",
          titleRequired: "默认语言标题不能为空",
          deleteConfirm: "确认删除文章「{name}」？",
          sectionAssets: "资源",
          assetsImmediateEffect: "注意：修改封面或附件会立即生效，无需再次保存文章",
          assetsNeedSave: "请先保存文章后再上传封面与附件",
          cover: "封面图",
          coverHint: "全语言共用封面，仅支持图片，最多 {max} 张（可在字典 post_cover_max 调整）",
          coverLimitReached: "已达封面图上限（{max} 张）",
          attachments: "附件",
          attachmentsHint: "可上传多个附件，支持图片、视频、压缩包与 Office 文档等",
          uploadCover: "上传封面",
          uploadAttachment: "上传附件",
          removeOnly: "仅从本文移除",
          removeAndPurge: "移除并删除文件",
          removeCoverConfirm: "移除封面？",
          removeAttachmentConfirm: "移除附件「{name}」？",
        },
      },
      category: {
        title: "分类管理",
        subtitle: "维护文章分类，支持多语言名称与描述",
        previewLang: "预览语言",
        name: "名称",
        description: "描述",
        sort: "排序",
        create: "新建分类",
        edit: "编辑分类",
        deleteConfirm: "确认删除分类「{name}」？",
        nameRequired: "默认语言名称不能为空",
        namePh: "分类名称",
        descPh: "分类描述（可选）",
        empty: "暂无分类",
      },
    },
    assets: {
      auth: "资源管理",
      title: "资源管理",
      subtitle: "统一管理上传的图片与附件，可按用途筛选",
      purpose: "用途",
      purposeAll: "全部",
      purposeCover: "封面图",
      purposeContent: "正文插图",
      purposeBanner: "轮播图",
      purposeAttachment: "附件",
      keyword: "文件名",
      keywordPh: "搜索原文件名或存储名",
      uploadName: "原文件名",
      storageName: "存储文件名",
      name: "文件名",
      preview: "预览",
      size: "大小",
      refs: "引用",
      upload: "上传",
      uploadPick: "选择文件",
      deleteConfirm: "确认彻底删除资源「{name}」？",
      deleteBlocked: "资源仍被引用，无法删除",
      empty: "暂无资源",
      createdAt: "上传时间",
      pickerTitle: "选择资源",
      pickerSelectTab: "从资源库选择",
      pickerUploadTab: "上传新文件",
      pickerConfirm: "确认选择",
      pickerOpen: "选择或上传图片",
      pickerReplace: "更换图片",
      pickerPurposeHint: "列表与上传均仅限当前用途",
      pickerUploadHint: "上传后将自动选中并关联到当前项",
    },
    banner: {
      auth: "轮播图",
      groups: "轮播图组",
      list: "轮播图列表",
      manage: {
        title: "轮播图管理",
        subtitle: "左侧管理轮播图组，右侧维护组内轮播图；公开 API 按组标识码加载",
        groups: "轮播图组",
        addBanner: "新增轮播图",
        addFirst: "添加第一张轮播图",
        selectGroup: "请先选择或创建一个轮播图组",
        currentGroup: "当前组：{name}（{code}）",
      },
      groupsDesc: "按展示位置划分轮播图组，前台通过标识码按组加载",
      groupsEmpty: "暂无轮播图组",
      groupCreate: "新建轮播图组",
      groupEdit: "编辑轮播图组",
      groupDeleteConfirm: "确认删除轮播图组「{name}」？",
      listDesc: "管理各组下的轮播图条目",
      listEmpty: "暂无轮播图",
      groupFilter: "轮播图组",
      groupFilterPh: "请选择轮播图组",
      group: "所属组",
      groupRequired: "请选择轮播图组",
      title: "标题",
      titlePh: "轮播图标题",
      titleRequired: "请输入标题",
      image: "图片",
      imagePh: "通过上传关联资源",
      imageHint: "仅支持图片；可先选图再保存，或保存后关联",
      assetsNeedSave: "新建时可先选图，保存轮播图后将自动关联",
      pendingAssetHint: "已选图片，请点击确定保存轮播图以完成关联",
      pendingAssetSaved: "图片已选中，保存轮播图后将自动关联",
      uploadImage: "上传图片",
      removeImageConfirm: "移除轮播图图片？",
      removeOnly: "仅解除关联",
      removeAndPurge: "解除并删除文件",
      imageRequired: "请上传轮播图图片",
      imageEnabled: "图片展示",
      pickerTitle: "选择轮播图",
      link: "链接",
      linkPh: "跳转链接（可选）",
      create: "新建轮播图",
      edit: "编辑轮播图",
      deleteConfirm: "确认删除轮播图「{name}」？",
    },
    menu: {
      auth: "菜单管理",
      tree: "站点菜单",
      manage: {
        title: "菜单管理",
        subtitle: "管理公开站导航、菜单组与多语言路径；后台侧栏为内置只读预览",
        groups: "菜单组",
        previewLang: "预览语言",
        addRoot: "新增顶级菜单",
        addChild: "子菜单",
        addFirst: "添加第一个菜单",
        emptyTree: "该菜单组暂无菜单项",
        groupsEmpty: "暂无菜单组",
        selectGroup: "请先选择或创建一个菜单组",
        readonly: "内置",
        readonlyHint: "系统内置菜单组，仅可查看结构，不可编辑",
        sidebarReadonly: "后台侧边栏为系统内置，菜单结构在代码中维护，此处仅供预览",
        untitled: "未命名",
        groupCreate: "新建菜单组",
        groupEdit: "编辑菜单组",
        groupName: "名称",
        groupNamePh: "如：页头菜单",
        groupNameRequired: "请输入菜单组名称",
        groupCode: "标识码",
        groupCodePh: "如：site_header",
        groupCodeRequired: "请输入标识码",
        groupCodePattern: "小写字母开头，仅含小写字母、数字、下划线",
        groupDesc: "描述",
        groupDeleteConfirm: "确认删除菜单组「{name}」？组内须无菜单项",
        group: "所属组",
        parent: "父菜单",
        parentRoot: "无（顶级菜单）",
        sectionStructure: "结构与权限",
        sectionI18n: "多语言文案",
        icon: "图标",
        iconPh: "emoji 或图片 URL",
        permission: "权限码",
        permissionPh: "留空表示不校验",
        sort: "排序",
        status: "状态",
        enabled: "启用",
        disabled: "禁用",
        itemCreate: "新建菜单",
        itemEdit: "编辑菜单",
        itemTitle: "标题",
        itemTitlePh: "菜单显示名称",
        itemTitleRequired: "默认语言标题不能为空",
        itemPath: "路径",
        itemDeleteConfirm: "确认删除菜单「{name}」？子菜单将一并删除",
      },
    },
    dict: {
      auth: "字典管理",
      manage: {
        title: "字典管理",
        subtitle: "维护站点级配置项，支持单值与多语言值",
        code: "标识码",
        label: "名称",
        description: "说明",
        translatable: "多语言",
        sort: "排序",
        previewLang: "预览语言",
        previewValue: "当前值",
        values: "字典值",
        create: "新建字典",
        edit: "编辑字典",
        deleteConfirm: "确认删除字典「{name}」？",
        codeRequired: "请输入标识码",
        labelRequired: "请输入名称",
        codePh: "如 site_icp",
        codePattern: "小写字母开头，仅含小写字母、数字、下划线",
        valuePh: "请输入字典值",
        yes: "是",
        no: "否",
      },
    },
    system: {
      auth: "系统管理",
      users: "用户管理",
      roles: "角色管理",
      user: {
        name: "用户管理",
        search: {
          label: {
            loginName: "登录账号",
            userName: "用户名称",
            phone: "手机号",
            deptId: "部门",
            loginTime: "登录时间",
          },
          placeholder: {
            loginName: "请输入登录账号",
            userName: "请输入用户名称",
            phone: "请输入手机号",
            deptId: "请选择部门",
          }
        },
        table: {
          loginName: "登录账号",
          deptName: "部门名称",
          avatar: "头像",
          userName: "用户名称",
          email: "邮箱",
          phone: "手机号",
          userType: "用户类型",
          sex: "用户性别",
          userStatus: "用户状态",
          loginTime: "登录时间"
        },
        form: {
          label: {
            loginName: "登录账号",
            password: "登录密码",
            userName: "用户名称",
            deptId: "部门名称",
            postId: "分配岗位",
            roleId: "分配角色",
            userType: "用户类型",
            userStatus: "用户状态",
            sex: "用户性别",
            avatar: "用户头像",
            phone: "手机号",
            email: "邮箱",
            remark: "用户备注"
          },
          placeholder: {
            loginName: "请输入登录账号",
            password: "请输入登录密码",
            userName: "请输入用户名称",
            deptId: "请选择部门",
            postId: "请选择岗位",
            roleId: "请选择角色",
            userType: "请选择用户类型",
            userStatus: "请选择用户状态",
            sex: "请选择用户性别",
            avatar: {
              description: "请上传头像",
              tip: "图片最大为 3M"
            },
            phone: "请输入手机号码",
            email: "请输入邮箱",
            remark: "请输入用户备注"
          }
        },
        rules: {
          loginName: { required: "请输入登录名称" },
          password: { required: "请输入用户密码", validator: "至少6位且包含字母和数字" },
          userName: { required: "请输入用户名字"},
          deptId: { required: "请选择用户部门" },
          userType: { required: "请输入用户类型" },
          sex: { required: "请选择用户性别" },
          userStatus: { required: "请选择用户状态" },
          phone: { required: "请输入手机号码" }
        },
        transfer: {
          role: "角色列表",
          post: "岗位列表"
        },
        manage: {
          title: "用户管理",
          subtitle: "管理系统账号、状态与角色分配",
          username: "用户名",
          nickname: "昵称",
          email: "邮箱",
          status: "状态",
          roles: "角色",
          password: "密码",
          passwordOptional: "留空则不修改",
          passwordRequired: "请输入密码",
          usernameRequired: "请输入用户名",
          create: "新建用户",
          edit: "编辑用户",
          deleteConfirm: "确认删除用户「{name}」？",
          enabled: "启用",
          disabled: "禁用",
          rolePlaceholder: "请选择角色",
        },
      },
      role: {
        name: "角色管理",
        manage: {
          title: "角色管理",
          subtitle: "配置角色名称、描述与权限集合",
          name: "角色名称",
          description: "描述",
          permCount: "权限数",
          permissions: "权限配置",
          create: "新建角色",
          edit: "编辑角色",
          deleteConfirm: "确认删除角色「{name}」？",
          nameRequired: "请输入角色名称",
          selectedCount: "已选择 {count} 项权限",
        },
      },
      menu: {
        name: "菜单管理"
      },
      dictType: {
        name: "字典管理"
      },
      dictData: {
        name: "字典详情"
      },
      dept: {
        name: "部门管理"
      },
      post: {
        name: "岗位管理"
      },
      loginLogs: {
        name: "登录日志"
      },
      operateLogs: {
        name: "操作日志"
      },
      notice: {
        name: "通知公告"
      },
      personage: {
        name: "个人中心"
      }
    },
    monitor: {
      auth: "系统监控",
      scheduled: {
        name: "定时任务"
      },
      online: {
        name: "在线用户"
      },
      service: {
        name: "服务监控"
      },
      redis: {
        name: "Redis监控"
      },
      cache: {
        name: "数据缓存"
      },
      blocklist: {
        name: "阻止名单"
      }
    },
    tools: {
      auth: "系统工具",
      generate: {
        name: "代码生成"
      },
      config: {
        name: "代码配置"
      },
      file: {
        name: "文件管理"
      },
      picture: {
        name: "图库管理"
      },
      testDept: {
        name: "测试部门"
      },
      testParams: {
        name: "测试参数"
      }
    },
    link: {
      auth: "外部链接",
      back: {
        name: "前后端"
      },
      front: {
        name: "纯前端"
      },
      blog: {
        name: "博客版本"
      },
      element: {
        name: "ElementPlus"
      }
    },
    blog: {
      auth: "博客管理",
      category: {
        name: "文章类别"
      },
      tag: {
        name: "标签管理"
      },
      article: {
        name: "文章管理"
      },
      friend: {
        name: "友链管理"
      },
      circle: {
        name: "朋友圈"
      },
      danMu: {
        name: "弹幕管理"
      },
      notice: {
        name: "通知公告"
      },
      library: {
        name: "知识库管理"
      },
      libraryCatalog: {
        name: "知识库目录"
      },
      libraryPreview: {
        name: "知识库预览"
      },
      comment: {
        name: "评论管理"
      },
    },
    coding: {
      404: {
        name: "404 页面"
      },
      403: {
        name: "403 页面"
      },
      500: {
        name: "500 页面"
      }
    }
  },
  button: {
    search: "搜索",
    reset: "重置",
    add: "添加",
    update: "修改",
    delete: "删除",
    export: "导出",
    import: "导入",
    preview: "预览",
    password: "重置密码",
    expand: "展开/折叠",
    role: "分配角色",
    post: "分配岗位",
    menu: "分配菜单",
    dept: "分配部门",
    refreshCache: "刷新缓存",
    view: "查看",
    detail: "详情",
    save: "保存",
    force: "强退",
    logout: "注销",
    execute: "执行",
    executeOnce: "执行一次",
    file: "文件上传",
    image: "图片上传",
    upload: "上传",
    download: "下载",
    confirm: "确定",
    cancel: "取消",
    refresh: "刷新",
    hideSearch: "隐藏搜索",
    displaySearch: "显示搜索",
    close: "关闭",
    genCode: "生成代码",
    sync: "同步",
    switch: "切换",
    publish: "发布",
    catalog: "目录",
    minimize: "收起窗口",
    restoreMinimized: "恢复表单窗口"
  },
  home: {
    welcome: "欢迎使用"
  },
  tabs: {
    refresh: "重新刷新",
    maximize: "全屏切换",
    exitMaximize: "退出全屏",
    closeCurrent: "关闭当前",
    closeLeft: "关闭左侧",
    closeRight: "关闭右侧",
    closeOther: "关闭其它",
    closeAll: "关闭所有",
    affix: "固定标签",
    unaffix: "取消固定"
  },
  header: {
    searchMenu: "搜索菜单",
    componentSize: "组件大小",
    refreshCache: "刷新缓存",
    lightMode: "明亮模式",
    darkMode: "暗黑模式",
    language: "语言翻译",
    fullScreen: "全屏",
    exitFullScreen: "退出全屏",
    collapseToolbar: "收起工具栏",
    expandToolbar: "展开工具栏",    
    settings: "设置",
    personalCenter: "个人中心",
    changePassword: "修改密码",
    logout: "退出登录",
    dimensionList: {
      default: "默认",
      large: "大型",
      small: "小型"
    },
    languageList: {
      chinese: "简体中文",
      english: "英文"
    },
    menuSearch: "菜单搜索：支持菜单名称、路径",
    searchMenuHint: "输入菜单名称或路径，快速定位页面",
    searchMenuSelect: "选择",
    searchMenuEnter: "确认",
    searchMenuEsc: "关闭"
  },
  msg: {
    success: "操作成功",
    fail: "操作失败，请刷新重试",
    selectData: "请选择数据",
    validFail: "验证失败，请检查表单内容",
    null: "暂无数据",
    closeTips: "您确认进行关闭么？",
    closed: "已关闭",
    cancelled: "已取消",
    remind: "温馨提示：",
    confirmWant: "您确认要",
    confirmDelete: "您确认要删除么？",
    confirmLogin: "账号身份已过期，请重新登录",
    selectDate: "请选择日期",
    selectDateTime: "请选择日期时间",
    selectNumber: "请输入数字",
    beginTime: "开始日期",
    endTime: "结束日期",
    to: "至",
    keyword: "关键字搜索",
    configFail: "配置失败",
    logIn: "请重新登录",
    yzmFail: "验证码获取失败"
  },
  table: {
    number: "序号",
    operate: "操作"
  },
  tree: {
    topLevel: "最顶级数据",
    selectParent: "请选择上级数据"
  },
  dict: {
    sys_switch_status: {
      open: "启用",
      stop: "停用",
    },
    sys_user_sex: {
      man: "男",
      woman: "女",
      unknown: "未知"
    },
    sys_yes_no: {
      yes: "是",
      no: "否"
    }
  }
};
