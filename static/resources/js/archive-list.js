/**
 * 公开站归档列表共用工具（默认模板列表 / 全部文章）
 */
(function (global) {
  "use strict";

  function escapeHtml(text) {
    return $("<span>").text(text).html();
  }

  function formatDisplayTime(ts, lang) {
    if (!ts) return "";
    var d = new Date(ts * 1000);
    return d.toLocaleString(lang === "en-us" ? "en-US" : "zh-CN", {
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  function coverUrl(post) {
    if (post.covers && post.covers.length > 0) return post.covers[0].url;
    if (post.attachments && post.attachments.length > 0) {
      var img = post.attachments.find(function (a) {
        return a.mime_type && a.mime_type.indexOf("image/") === 0;
      });
      if (img) return img.url;
    }
    return null;
  }

  function postDetailUrl(post, localePrefix) {
    if (post.route_path) return post.route_path;
    return localePrefix + "/posts/" + post.id;
  }

  function parseTagChips(tagsRaw) {
    if (!tagsRaw || !tagsRaw.trim()) return "";
    var tags = tagsRaw.split(/[,，]/).map(function (s) {
      return s.trim();
    }).filter(Boolean);
    if (!tags.length) return "";
    return tags.map(function (tag) {
      return '<span class="archive-chip">' + escapeHtml(tag) + "</span>";
    }).join("");
  }

  /** 列表排序：置顶优先，同组内按展示时间新→旧 */
  function sortPostsForList(posts) {
    return posts.slice().sort(function (a, b) {
      var pinA = a.pinned ? 1 : 0;
      var pinB = b.pinned ? 1 : 0;
      if (pinB !== pinA) return pinB - pinA;
      return (b.display_time || 0) - (a.display_time || 0);
    });
  }

  /**
   * 渲染默认归档卡片
   * @param {object} post
   * @param {object} opts - { localePrefix, lang, maxSummary, showCategory }
   */
  function renderArchiveCard(post, opts) {
    var localePrefix = opts.localePrefix || "";
    var lang = opts.lang || "zh-cn";
    var maxSummary = opts.maxSummary || 200;
    var detailUrl = postDetailUrl(post, localePrefix);
    var thumb = coverUrl(post);

    var summary = post.description || "";
    if (!summary && post.content && global.stripMarkdown) {
      summary = global.stripMarkdown(post.content);
    }
    if (summary && summary.length > maxSummary) {
      summary = summary.substring(0, maxSummary) + "…";
    }

    var cardClass = "archive-card";
    if (thumb) cardClass += " archive-card--media";

    var html = '<li class="' + cardClass + '">';
    html += '<a class="archive-card__link" href="' + detailUrl + '">';

    if (thumb) {
      html += '<div class="archive-card__media"><img src="' + thumb + '" alt="" loading="lazy"></div>';
    }

    html += '<div class="archive-card__body">';
    if (post.display_time) {
      html += '<time class="archive-card__date">' + escapeHtml(formatDisplayTime(post.display_time, lang)) + "</time>";
    }
    html += '<h2 class="archive-card__title">' + escapeHtml(post.title) + "</h2>";
    if (summary) {
      html += '<p class="archive-card__summary">' + escapeHtml(summary) + "</p>";
    }

    var foot = "";
    if (opts.showCategory && post.category_name) {
      foot += '<span class="archive-chip archive-chip--secondary">' + escapeHtml(post.category_name) + "</span>";
    }
    foot += parseTagChips(post.tags);
    if (foot) {
      html += '<div class="archive-card__foot">' + foot + "</div>";
    }

    html += "</div>";
    html += '<span class="archive-card__arrow" aria-hidden="true">→</span>';
    html += "</a></li>";
    return html;
  }

  global.ArchiveList = {
    escapeHtml: escapeHtml,
    formatDisplayTime: formatDisplayTime,
    coverUrl: coverUrl,
    postDetailUrl: postDetailUrl,
    sortPostsForList: sortPostsForList,
    renderArchiveCard: renderArchiveCard,
  };
})(window);
