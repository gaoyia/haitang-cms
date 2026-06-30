export default {
  project: {
    title: "KOI-ADMIN"
  },
  common: {
    publicUrlLinkId: "ID",
    publicUrlLinkSeo: "SEO",
    copyLink: "Copy link",
    copyLinkSuccess: "Link copied",
    copyLinkFailed: "Copy failed",
  },
  menu: {
    login: {
      auth: "Login",
      title: "KOI-ADMIN Platform",
      welcome: "Welcome to login",
      platform: "Management platform",
      description: "Maybe we just got lucky",
      account: "Account password login",
      in: "Log in",
      loading: "Be logging in",
      beiAnHao: "Website record number",
      picture: "I can't see it. Change it",
      form: {
        loginName: "Please enter your username",
        password: "Please enter password",
        securityCode: "Please enter the verification code",
      },
      rules: {
        loginName: {
          required: "The user name cannot be empty",
          validator: "The account can only contain numbers and letters"
        },
        password: {
          required: "The password cannot be empty",
          validator1: "6 to 20 characters in length",
          validator2: "The password must contain both numbers and letters",
        },
        securityCode: {
          required: "The verification code cannot be empty"
        }
      }
    },
    home: {
      auth: "Master Station",
      welcome: "Welcome to Haitang CMS",
      welcomeBack: "Welcome back",
      greetingFallback: "Hello, hope you have a great day!",
      defaultUser: "Admin",
      subtitle: "Content platform — efficient, elegant, reliable",
      quickStart: "Quick links",
      quickStartDesc: "Jump to frequently used modules",
      aboutTitle: "About",
      aboutDesc: "{name} is a Rust + Vue 3 CMS. The public site uses Tera templates; the admin is a standalone SPA. Live stats will appear here once modules are connected.",
      statPending: "Stats pending",
      linkPosts: "Create and manage posts",
      linkBanners: "Configure site banners",
      linkMenus: "Maintain navigation menus",
      linkUsers: "Manage users and access",
      work: {
        name: "Workbench page"
      },
      analysis: {
        name: "Analysis page"
      },
      console: {
        name: "Console Page"
      }
    },
    content: {
      auth: "Content",
      posts: "Posts",
      categories: "Categories",
      post: {
        manage: {
          title: "Posts",
          subtitle: "Create and manage multilingual posts",
          filterCategory: "Category",
          filterCategoryAll: "All",
          previewLang: "Preview language",
          titleCol: "Title",
          category: "Category",
          categoryPh: "Select category",
          categoryRequired: "Category is required",
          categoryTemplateHint: "Category detail template: {template}",
          categoryTemplateDefault: "Default",
          categoryTemplateGallery: "Gallery",
          categoryTemplateRecruitment: "Recruitment",
          categoryTemplateAbout: "About Us",
          status: "Status",
          draft: "Draft",
          published: "Published",
          scheduled: "Scheduled",
          displayTime: "Display time",
          displayTimePh: "Empty uses server time",
          displayTimeHint: "Leave empty to use server time on save",
          publishTime: "Publish time",
          publishTimePh: "Empty publishes immediately when published",
          publishTimeHint: "Set a future time to schedule; empty + published status publishes now",
          routePath: "Path",
          publicUrl: "Public URL",
          publicUrlHint: "The URL visitors use today; the post ID is assigned after save",
          publicUrlPending: "After save",
          openPublic: "Open",
          seoPath: "SEO path (optional)",
          seoPathPh: "my-slug",
          seoPathHint: "Enter the slug only; the prefix is fixed. Leave empty for now — slug routing is not enabled yet",
          seoPathSlugError: "Slug cannot be empty or contain spaces, #, or ?",
          tags: "Tags",
          tagsPh: "Press Enter to add tags; localize per language",
          tagExtractCount: "Count",
          tagExtract: "Extract tags",
          tagExtractLlm: "LLM extract",
          tagExtractLlmPending: "LLM keyword extraction is not available yet",
          tagExtractEmpty: "Add body content before extracting tags",
          tagExtractNoResult: "No suitable keywords found in the current text",
          tagExtractSuccess: "Added {count} tag(s)",
          tagExtractFail: "Failed to load Jieba segmenter; please try again later",
          description: "Summary",
          descriptionPh: "Post summary",
          content: "Body",
          contentPh: "Markdown supported; use preview and fullscreen while editing",
          sectionMeta: "Meta",
          sectionRecruitmentMeta: "Recruitment fields",
          recruitmentSalary: "Salary",
          recruitmentSalaryPh: "e.g. 15K-25K",
          recruitmentLocation: "Location",
          recruitmentLocationPh: "e.g. Beijing / Remote",
          recruitmentEmploymentType: "Employment type",
          recruitmentEmploymentTypePh: "e.g. Full-time / Intern",
          recruitmentDepartment: "Department",
          recruitmentDepartmentPh: "e.g. Engineering",
          sectionAboutMeta: "About page extras",
          aboutHighlight: "Highlight",
          aboutHighlightPh: "e.g. Structured elegance",
          aboutFounded: "Founded",
          aboutFoundedPh: "e.g. 2024",
          aboutLocation: "Location",
          aboutLocationPh: "e.g. Beijing",
          aboutContact: "Contact",
          aboutContactPh: "{'e.g. hello@example.com'}",
          sectionI18n: "Translations",
          create: "New post",
          edit: "Edit post",
          titlePh: "Post title",
          titleRequired: "Default locale title is required",
          deleteConfirm: "Delete post \"{name}\"?",
          sectionAssets: "Assets",
          assetsImmediateEffect: "Note: Changes to cover or attachments take effect immediately; no need to save the post again",
          assetsNeedSave: "You may pick cover and attachments first; they link automatically after save",
          pendingAssetsHint: "Selected assets are not linked yet; save the post to finish",
          pendingAssetSaved: "Asset selected; it will link when you save the post",
          assetAlreadyLinked: "This asset is already linked to this post",
          cover: "Cover",
          coverHint: "Covers are shared across locales; images only; up to {max}; drag handle to reorder",
          coverDrag: "Drag to reorder",
          coverLimitReached: "Cover limit reached ({max})",
          attachments: "Attachments",
          attachmentsHint: "Multiple attachments; drag the handle to reorder",
          attachmentsDrag: "Drag to reorder",
          selectCover: "Select cover",
          selectAttachment: "Select attachment",
          pickerCoverTitle: "Select cover image",
          pickerAttachmentTitle: "Select attachment",
          removeOnly: "Remove from this post only",
          removeAndPurge: "Remove and delete file",
          removeCoverConfirm: "Remove cover?",
          removeAttachmentConfirm: "Remove attachment \"{name}\"?",
          removeAttachmentsConfirm: "Remove {count} selected attachments?",
          attachmentsSelectedCount: "{count} selected",
        },
      },
      category: {
        title: "Categories",
        subtitle: "Manage categories with multilingual SEO paths and list/detail templates",
        previewLang: "Preview language",
        name: "Name",
        description: "Description",
        sort: "Sort",
        create: "New category",
        edit: "Edit category",
        deleteConfirm: "Delete category \"{name}\"?",
        nameRequired: "Default locale name is required",
        namePh: "Category name",
        descPh: "Description (optional)",
        empty: "No categories",
        listTemplate: "List template",
        detailTemplate: "Detail template",
        templateDefault: "Default",
        templateGallery: "Gallery",
        templateRecruitment: "Recruitment",
        templateAbout: "About Us",
        templateNone: "None",
        listTemplateNoneHint: "With no list template, the category archive URL returns 404. Link to post detail pages or set a direct menu URL instead.",
        seoPath: "SEO path (optional)",
        seoPathPh: "albums",
        seoPathHint: "Slug only; prefix is /{lang}/categories/; empty uses numeric ID",
        seoPathSlugError: "Slug cannot be empty or contain spaces, #, ?, or /",
        publicUrl: "Public URL",
        openPublic: "Open",
      },
    },
    assets: {
      auth: "Assets",
      title: "Assets",
      subtitle: "Manage uploaded images and attachments",
      purpose: "Purpose",
      purposeAll: "All",
      purposeCover: "Cover",
      purposeContent: "Content image",
      purposeBanner: "Banner",
      purposeFriendLink: "Friend link",
      purposeAttachment: "Attachment",
      keyword: "Filename",
      keywordPh: "Search upload or storage name",
      uploadName: "Original filename",
      storageName: "Storage filename",
      name: "Filename",
      preview: "Preview",
      size: "Size",
      refs: "Refs",
      upload: "Upload",
      uploadPick: "Choose file",
      uploadMultiPick: "Choose multiple",
      deleteConfirm: "Permanently delete \"{name}\"?",
      deleteBlocked: "Asset is still referenced",
      empty: "No assets",
      createdAt: "Uploaded",
      pickerTitle: "Select asset",
      pickerSelectTab: "From library",
      pickerUploadTab: "Upload new",
      pickerConfirm: "Confirm",
      pickerConfirmCount: "Confirm ({count})",
      pickerSelectedCount: "{count} selected",
      pickerSelectMultiHint: "Click to toggle selection; confirm to add all selected",
      pickerAlreadyLinked: "Already linked",
      pickerOpen: "Select or upload",
      pickerReplace: "Replace",
      pickerPurposeHint: "List and upload are limited to this purpose",
      pickerUploadHint: "Files link automatically; the dialog stays open so you can upload more or pick from the library",
    },
    banner: {
      auth: "Banners",
      groups: "Banner groups",
      list: "Banner list",
      manage: {
        title: "Banner management",
        subtitle: "Manage groups on the left and banner items on the right; public API loads by group code",
        groups: "Groups",
        addBanner: "Add banner",
        addFirst: "Add first banner",
        selectGroup: "Select or create a banner group first",
        currentGroup: "Group: {name} ({code})",
        dragSort: "Drag to reorder",
      },
      groupsDesc: "Groups banners by placement; public API loads by group code",
      groupsEmpty: "No banner groups",
      groupCreate: "New banner group",
      groupEdit: "Edit banner group",
      groupDeleteConfirm: "Delete banner group \"{name}\"?",
      listDesc: "Manage banner items within each group",
      listEmpty: "No banners",
      groupFilter: "Banner group",
      groupFilterPh: "Select a group",
      group: "Group",
      groupRequired: "Please select a group",
      title: "Title",
      titlePh: "Banner title",
      titleRequired: "Title is required",
      image: "Image",
      imagePh: "Upload via asset library",
      imageHint: "Images only; pick before or after save",
      assetsNeedSave: "You may pick an image first; it links automatically after save",
      pendingAssetHint: "Image selected — click Confirm to save the banner and link it",
      pendingAssetSaved: "Image selected; it will link when you save the banner",
      uploadImage: "Upload image",
      removeImageConfirm: "Remove banner image?",
      removeOnly: "Unlink only",
      removeAndPurge: "Unlink and delete file",
      imageRequired: "Please upload a banner image",
      imageEnabled: "Show image",
      pickerTitle: "Select banner image",
      link: "Link",
      linkPh: "Link URL (optional)",
      create: "New banner",
      edit: "Edit banner",
      deleteConfirm: "Delete banner \"{name}\"?",
    },
    menu: {
      auth: "Menus",
      tree: "Site menus",
      groups: "Groups",
      manage: {
        title: "Menu management",
        subtitle: "Manage public navigation and groups; admin sidebar is built-in preview only",
        groups: "Menu groups",
        previewLang: "Preview language",
        addRoot: "Add root item",
        addChild: "Child",
        addFirst: "Add first item",
        emptyTree: "No menu items in this group",
        groupsEmpty: "No menu groups",
        selectGroup: "Select or create a menu group first",
        readonly: "Built-in",
        readonlyHint: "Built-in group is read-only",
        sidebarReadonly: "Admin sidebar is built-in; structure is maintained in code (preview only)",
        untitled: "Untitled",
        groupCreate: "New menu group",
        groupEdit: "Edit menu group",
        groupName: "Name",
        groupNamePh: "e.g. Header nav",
        groupNameRequired: "Name is required",
        groupCode: "Code",
        groupCodePh: "e.g. site_header",
        groupCodeRequired: "Code is required",
        groupCodePattern: "Lowercase letter first; letters, digits, underscore only",
        groupDesc: "Description",
        groupDeleteConfirm: "Delete group \"{name}\"? Group must be empty",
        group: "Group",
        parent: "Parent",
        parentRoot: "None (root)",
        sectionStructure: "Structure & permission",
        sectionI18n: "Translations",
        icon: "Icon",
        iconPh: "Emoji or image URL",
        permission: "Permission",
        permissionPh: "Empty = no check",
        sort: "Sort",
        status: "Status",
        enabled: "Enabled",
        disabled: "Disabled",
        itemCreate: "New menu item",
        itemEdit: "Edit menu item",
        itemTitle: "Title",
        itemTitlePh: "Display name",
        itemTitleRequired: "Default locale title is required",
        itemPath: "Path",
        itemPathRequired: "Default locale path is required",
        itemPathPhAuto: "/about",
        autoLocalePrefix: "Auto locale prefix",
        autoLocalePrefixHint: "When enabled, enter paths like /about; /zh-cn and /en-us prefixes are added on save",
        itemDeleteConfirm: "Delete \"{name}\" and its children?",
        dragSort: "Drag to reorder",
        treeDragHint: "Drag to reorder; drop onto a node to nest under it, or before/after to reorder siblings (like VS Code)",
      },
    },
    dict: {
      auth: "Dictionary",
      manage: {
        title: "Dictionary",
        subtitle: "Manage site-wide config entries with optional i18n values",
        code: "Code",
        label: "Label",
        description: "Description",
        translatable: "Translatable",
        sort: "Sort",
        previewLang: "Preview language",
        previewValue: "Current value",
        values: "Values",
        create: "New dictionary",
        edit: "Edit dictionary",
        deleteConfirm: "Delete dictionary \"{name}\"?",
        codeRequired: "Code is required",
        labelRequired: "Label is required",
        codePh: "e.g. site_icp",
        codePattern: "Lowercase letter first; letters, digits, underscore only",
        valuePh: "Enter value",
        yes: "Yes",
        no: "No",
      },
    },
    system: {
      auth: "System",
      users: "Users",
      roles: "Roles",
      user: {
        name: "User Manage",
        search: {
          label: {
            loginName: "Login name",
            userName: "User name",
            phone: "Phone number",
            deptId: "Department",
            loginTime: "Login time",  
          },
          placeholder: {
            loginName: "Please enter your login account",
            userName: "Please enter the user name",
            phone: "Please enter your phone number",
            deptId: "Please select a department",
          }
        },
        table: {
          loginName: "Login name",
          deptName: "Department",
          avatar: "Avatar",
          userName: "User name",
          email: "Email",
          phone: "Phone number",
          userType: "User type",
          sex: "Gender",
          userStatus: "User status",
          loginTime: "Login time"
        },
        form: {
          label: {
            loginName: "Login name",
            password: "Password",
            userName: "User name",
            deptId: "Department",
            postId: "Job allocation",
            roleId: "Assign roles",
            userType: "User type",
            userStatus: "User status",
            sex: "Gender",
            avatar: "Avatar",
            phone: "Phone number",
            email: "Email",
            remark: "Remark"
          },
          placeholder: {
            loginName: "Please enter your login account",
            password: "Please enter your password",
            userName: "Please enter the user name",
            deptId: "Please select a department",
            postId: "Please select the position",
            roleId: "Please select a role",
            userType: "Please select the user type",
            userStatus: "Please select the user status",
            sex: "Please select the user's gender",
            avatar: {
              description: "Upload an avatar",
              tip: "The maximum size of the picture is 3M"
            },
            phone: "Please enter your phone number",
            email: "Please enter your email address",
            remark: "Please enter the user's remarks"
          }
        },
        rules: {
          loginName: { required: "Please enter your login account" },
          password: { required: "Please enter your user password", validator: "At least 6 digits and containing letters and numbers" },
          userName: { required: "Please enter the user name"},
          deptId: { required: "Please select the department" },
          userType: { required: "Please enter the user type" },
          sex: { required: "Please select the user's gender" },
          userStatus: { required: "Please select the user status" },
          phone: { required: "Please enter your phone number" }
        },
        transfer: {
          role: "List of roles",
          post: "Job list"
        },
        manage: {
          title: "Users",
          subtitle: "Manage accounts, status and role assignments",
          username: "Username",
          nickname: "Nickname",
          email: "Email",
          status: "Status",
          roles: "Roles",
          password: "Password",
          passwordOptional: "Leave blank to keep unchanged",
          passwordRequired: "Password is required",
          usernameRequired: "Username is required",
          create: "New user",
          edit: "Edit user",
          deleteConfirm: "Delete user \"{name}\"?",
          enabled: "Enabled",
          disabled: "Disabled",
          rolePlaceholder: "Select roles",
        },
      },
      role: {
        name: "Role Manage",
        manage: {
          title: "Roles",
          subtitle: "Configure role name, description and permissions",
          name: "Role name",
          description: "Description",
          permCount: "Permissions",
          permissions: "Permission config",
          create: "New role",
          edit: "Edit role",
          deleteConfirm: "Delete role \"{name}\"?",
          nameRequired: "Role name is required",
          selectedCount: "{count} permission(s) selected",
        },
      },
      menu: {
        name: "Menu Manage"
      },
      dictType: {
        name: "DictType Manage"
      },
      dictData: {
        name: "DictData Manage"
      },
      dept: {
        name: "Dept Manage"
      },
      post: {
        name: "Post Manage"
      },
      loginLogs: {
        name: "Login Logs"
      },
      operateLogs: {
        name: "Operate Logs"
      },
      notice: {
        name: "Notice Manage"
      },
      personage: {
        name: "Personage Center"
      }
    },
    monitor: {
      auth: "System Monitor",
      scheduled: {
        name: "Scheduled Task"
      },
      online: {
        name: "Online User"
      },
      service: {
        name: "Service Monitor"
      },
      redis: {
        name: "Redis Monitor"
      },
      cache: {
        name: "Data Cache"
      },
      blocklist: {
        name: "Blocklist Manage"
      }
    },
    tools: {
      auth: "System Tool",
      generate: {
        name: "Code Generate"
      },
      config: {
        name: "Code Config"
      },
      file: {
        name: "Files Manage"
      },
      picture: {
        name: "Pictures Manage"
      },
      testDept: {
        name: "TestDept Manage"
      },
      testParams: {
        name: "TestDept Params"
      }
    },
    link: {
      auth: "External Link",
      back: {
        name: "Back Version"
      },
      front: {
        name: "Front Version"
      },
      blog: {
        name: "Blog Version"
      },
      element: {
        name: "ElementPlus"
      }
    },
    blog: {
      auth: "Blog Manage",
      category: {
        name: "Article Category"
      },
      tag: {
        name: "Tags Manage"
      },
      article: {
        name: "Article Manage"
      },
      friend: {
        name: "Friend Link"
      },
      circle: {
        name: "Circle Manage"
      },
      danMu: {
        name: "DanMu Manage"
      },
      notice: {
        name: "Notice Manage"
      },
      library: {
        name: "Knowledge Base"
      },
      libraryCatalog: {
        name: "Knowledge Catalog"
      },
      libraryPreview: {
        name: "Knowledge Preview"
      },
      comment: {
        name: "Comment Manage"
      },
    },
    coding: {
      404: {
        name: "404 Page"
      },
      403: {
        name: "403 Page"
      },
      500: {
        name: "500 Page"
      }
    }
  },
  button: {
    search: "Search",
    reset: "Reset",
    add: "Add",
    update: "Update",
    delete: "Delete",
    export: "Export",
    import: "Import",
    preview: "Preview",
    password: "Reset Password",
    expand: "Expand/Fold",
    role: "Assign Roles",
    post: "Assign Jobs",
    menu: "Assign Menu",
    dept: "Assign Dept",
    refreshCache: "Refresh Cache",
    view: "View",
    detail: "Detail",
    save: "Save",
    force: "Force Offline",
    logout: "Logout",
    execute: "Execute",
    executeOnce: "Execute Once",
    file: "File Upload",
    image: "Image Upload",
    upload: "Upload",
    download: "Download",
    confirm: "Confirm",
    cancel: "Cancel",
    refresh: "Refresh",
    hideSearch: "Hide Search",
    displaySearch: "Display Search",
    close: "Close",
    genCode: "Generate Code",
    previewCode: "Preview Code",
    sync: "Sync",
    switch: "Switch",
    publish: "Publish",
    catalog: "Catalog",
    minimize: "Minimize",
    restoreMinimized: "Restore form window"
  },
  home: {
    welcome: "Welcome"
  },
  tabs: {
    refresh: "Refresh",
    maximize: "Maximize",
    exitMaximize: "Exit Maximize",
    closeCurrent: "Close Current",
    closeLeft: "Close Left",
    closeRight: "Close Right",
    closeOther: "Close Other",
    closeAll: "Close All",
    affix: "Affix Tab",
    unaffix: "Unfix Tab"
  },
  header: {
    searchMenu: "Search menu",
    componentSize: "Component size",
    refreshCache: "Refresh cache",
    lightMode: "Light mode",
    darkMode: "Dark mode",
    language: "Language translation",
    fullScreen: "Full Screen",
    exitFullScreen: "Exit Full Screen",
    collapseToolbar: "Collapse toolbar",
    expandToolbar: "Expand toolbar",    
    personalCenter: "Personal Center",
    settings: "Settings",
    logout: "Log out",
    dimensionList: {
      default: "default",
      large: "large",
      small: "small"
    },
    languageList: {
      chinese: "Chinese",
      english: "English"
    },
    menuSearch: "Menu search: Support menu name, path",
    searchMenuHint: "Type a menu name or path to navigate quickly",
    searchMenuSelect: "Navigate",
    searchMenuEnter: "Open",
    searchMenuEsc: "Close"
  },
  msg: {
    success: "Operation successful",
    fail: "Operation failed. Please refresh and try again",
    selectData: "Please select the data",
    validFail: "Validation failed. Please check the form contents",
    null: "No data for now",
    closeTips: "Are you sure you want to close it?",
    closed: "Closed",
    cancelled: "Cancelled",
    remind: "Friendly reminder:",
    confirmWant: "Do you confirm that you want",
    confirmDelete: "Are you sure you want to delete it?",
    confirmLogin: "The account identity has expired, please log in again",
    selectDate: "Please select a date",
    selectDateTime: "Please select a date and time",
    selectNumber: "Please enter the number",
    beginTime: "Begin Time",
    endTime: "End Time",
    to: "to",
    keyword: "Keyword search",
    configFail: "Configuration failed",
    logIn: "Please log in again",
    yzmFail: "Captcha acquisition failed"
  },
  table: {
    number: "#",
    operate: "Operation"
  },
  tree: {
    topLevel: "Top level data",
    selectParent: "Please select parent data"
  },
  dict: {
    sys_switch_status: {
      open: "open",
      stop: "stop",
    },
    sys_user_sex: {
      man: "man",
      woman: "woman",
      unknown: "unknown"
    },
    sys_yes_no: {
      yes: "yes",
      no: "no"
    }
  }
};
