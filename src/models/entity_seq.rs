/// 逻辑实体 ID 序列表（post / category / menu_item 等共用命名空间）
#[derive(Debug, toasty::Model)]
pub struct EntitySeq {
    #[key]
    pub name: String,

    pub next_id: i64,
}

/// 分配并递增指定实体的下一个逻辑 ID
pub async fn next_entity_id(db: &mut toasty::Db, name: &str) -> Result<i64, String> {
    let rows = EntitySeq::all()
        .exec(db)
        .await
        .map_err(|e| format!("查询序列表失败: {e}"))?;

    if let Some(mut row) = rows.into_iter().find(|r| r.name == name) {
        let id = row.next_id;
        row.update()
            .next_id(id + 1)
            .exec(db)
            .await
            .map_err(|e| format!("更新序列表失败: {e}"))?;
        return Ok(id);
    }

    EntitySeq::create()
        .name(name)
        .next_id(2)
        .exec(db)
        .await
        .map_err(|e| format!("初始化序列表失败: {e}"))?;

    Ok(1)
}
