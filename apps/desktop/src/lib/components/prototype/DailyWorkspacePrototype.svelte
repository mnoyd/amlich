<script lang="ts">
  import PrototypeSwitcher from './PrototypeSwitcher.svelte';

  export let variant: string;

  const hours = [
    { branch: 'Tý', time: '23–01', tone: 'support', note: 'Khởi sự nhẹ' },
    { branch: 'Sửu', time: '01–03', tone: 'quiet', note: 'Yên' },
    { branch: 'Dần', time: '03–05', tone: 'constraint', note: 'Có xung' },
    { branch: 'Mão', time: '05–07', tone: 'support', note: 'Minh Đường' },
    { branch: 'Thìn', time: '07–09', tone: 'quiet', note: 'Bình' },
    { branch: 'Tỵ', time: '09–11', tone: 'selected', note: 'Đang xét' },
    { branch: 'Ngọ', time: '11–13', tone: 'support', note: 'Thanh Long' },
    { branch: 'Mùi', time: '13–15', tone: 'quiet', note: 'Bình' },
    { branch: 'Thân', time: '15–17', tone: 'constraint', note: 'Thận trọng' },
    { branch: 'Dậu', time: '17–19', tone: 'support', note: 'Chuyển tiết' },
    { branch: 'Tuất', time: '19–21', tone: 'quiet', note: 'Yên' },
    { branch: 'Hợi', time: '21–23', tone: 'quiet', note: 'Yên' },
  ];

  const evidence = [
    { label: 'Lịch pháp', value: 'Đầy đủ', width: 100, status: 'ready' },
    { label: 'Ngày & sao', value: '8 / 8 nguồn', width: 100, status: 'ready' },
    { label: 'Theo mục đích', value: 'Chưa có mục đích', width: 36, status: 'missing' },
    { label: 'Theo hồ sơ', value: 'Chưa có hồ sơ', width: 18, status: 'missing' },
  ];

  const signals = {
    supports: [
      { title: 'Minh Đường hiện nhật', reason: 'Một tín hiệu ngày Hoàng Đạo được ghi nhận.' },
      { title: 'Trực Thành', reason: 'Ngày mang thế hoàn tất và định hình.' },
    ],
    constraints: [
      { title: 'Ngày Mùi xung Sửu', reason: 'Có quan hệ xung cần xét khi thêm hồ sơ cá nhân.' },
    ],
    unknowns: [
      { title: 'Chưa xét việc cụ thể', reason: 'Thêm mục đích để tạo Đánh giá Ngày.' },
    ],
  };

  const layers = ['Tổng quan', 'Giờ trong ngày', 'Bối cảnh cá nhân', 'Chứng cứ'];
</script>

<svelte:head>
  <title>Nguyên mẫu Không gian Ngày — Âm Lịch</title>
</svelte:head>

<div class="prototype-shell">
  <div class="prototype-flag">NGUYÊN MẪU · DỮ LIỆU MINH HỌA · KHÔNG PHẢI SẢN PHẨM</div>

  {#if variant === 'A'}
    <div class="variant-a">
      <header class="a-header">
        <div class="brand"><span>ÂM LỊCH</span><small>Đài quan sát ngày</small></div>
        <div class="date-stepper">
          <button aria-label="Ngày trước">‹</button>
          <div><b>Thứ bảy, 19 tháng 9</b><small>9 tháng 8, Bính Ngọ · ngày Ất Mùi</small></div>
          <button aria-label="Ngày sau">›</button>
        </div>
        <button class="today">Hôm nay · 09:42</button>
      </header>

      <div class="a-body">
        <main class="a-main">
          <section class="a-intro">
            <p class="eyebrow">Tổng quan ngày · Thu phân sau 3 ngày</p>
            <h1>Một ngày có thế <em>hoàn tất</em>, với một xung cần lưu ý.</h1>
            <p class="lead">Đây là Mẫu Ngày ẩn danh. Không có kết luận phù hợp hay không phù hợp cho đến khi bạn nêu việc muốn làm.</p>
            <button class="primary">+ Thêm mục đích để đánh giá ngày</button>
          </section>

          <section class="pattern-section">
            <div class="section-heading"><span>01</span><div><h2>Mẫu Ngày</h2><p>Tín hiệu được nhóm, không quy thành điểm tốt/xấu.</p></div></div>
            <div class="signal-grid">
              <article class="signal support">
                <h3>Hỗ trợ <span>2</span></h3>
                {#each signals.supports as signal}
                  <button class="signal-row"><b>{signal.title}</b><small>{signal.reason}</small><i>→</i></button>
                {/each}
              </article>
              <article class="signal constraint">
                <h3>Giới hạn <span>1</span></h3>
                {#each signals.constraints as signal}
                  <button class="signal-row"><b>{signal.title}</b><small>{signal.reason}</small><i>→</i></button>
                {/each}
              </article>
              <article class="signal unknown">
                <h3>Chưa biết <span>1</span></h3>
                {#each signals.unknowns as signal}
                  <button class="signal-row"><b>{signal.title}</b><small>{signal.reason}</small><i>→</i></button>
                {/each}
              </article>
            </div>
          </section>

          <section class="hours-section">
            <div class="section-heading"><span>02</span><div><h2>Giờ trong ngày</h2><p>Đủ mười hai cửa sổ; nổi bật không có nghĩa là phù hợp nhất.</p></div></div>
            <div class="hour-ribbon">
              {#each hours as hour}
                <button class:active={hour.tone === 'selected'} class:notable={hour.tone === 'support'} class:caution={hour.tone === 'constraint'}>
                  <small>{hour.time}</small><b>{hour.branch}</b><i>{hour.note}</i>
                </button>
              {/each}
            </div>
            <article class="selected-hour">
              <div class="time-mark"><small>Đang chọn</small><strong>09:42</strong><span>giờ Tỵ · 09–11</span></div>
              <div><h3>Khoảng giờ đang diễn ra</h3><p>Không có tín hiệu bất thường ở lớp ẩn danh. Thêm mục đích hoặc hồ sơ sẽ làm rõ các kết quả mới mà không thay đổi dữ kiện lịch.</p></div>
              <button>Xem lý do & chứng cứ →</button>
            </article>
          </section>
        </main>

        <aside class="a-aside">
          <section><p class="eyebrow">Lịch tính</p><div class="identity"><strong>Ất Mùi</strong><span>ngày</span></div><dl><div><dt>Năm</dt><dd>Bính Ngọ</dd></div><div><dt>Tháng</dt><dd>Đinh Dậu</dd></div><div><dt>Tiết khí</dt><dd>Bạch Lộ</dd></div></dl></section>
          <section><div class="aside-title"><h3>Độ phủ chứng cứ</h3><span>2 chưa biết</span></div>{#each evidence as item}<div class="coverage"><div><b>{item.label}</b><small>{item.value}</small></div><div class="bar"><i style={`width:${item.width}%`} class:missing={item.status === 'missing'}></i></div></div>{/each}<button class="text-button">Mở Trình khám phá chứng cứ →</button></section>
          <section class="context-card"><p class="eyebrow">Làm rõ cùng một ngày</p><h3>Bối cảnh cá nhân</h3><p>Thêm từng lớp khi cần. Tổng quan ẩn danh vẫn giữ nguyên.</p><button>+ Mục đích</button><button>+ Hồ sơ sinh</button><button>+ Địa điểm</button></section>
        </aside>
      </div>
    </div>

  {:else if variant === 'B'}
    <div class="variant-b">
      <header class="b-header"><div class="b-brand">ÂL<span>OBSERVATORY</span></div><div class="b-date"><button>←</button><div><small>THỨ BẢY</small><b>19.09.2026</b><span>09 tháng 08 · Bính Ngọ</span></div><button>→</button></div><div class="b-now"><i></i> Bây giờ · Tỵ 09:42</div></header>
      <div class="b-grid">
        <nav class="b-nav"><p>LỚP QUAN SÁT</p>{#each layers as layer, index}<button class:active={index === 1}><span>0{index + 1}</span>{layer}{#if index === 2}<i>+2</i>{/if}</button>{/each}<div class="b-profile"><small>CHẾ ĐỘ</small><b>Ẩn danh</b><button>Thêm bối cảnh +</button></div></nav>
        <main class="b-stage">
          <div class="b-summary"><div><p class="eyebrow">MẪU NGÀY · ẤT MÙI</p><h1>2 hỗ trợ <span>·</span> 1 giới hạn <span>·</span> 1 chưa biết</h1></div><button>Tạo Đánh giá Ngày +</button></div>
          <section class="b-timeline"><div class="timeline-head"><div><p class="eyebrow">TRỤC GIỜ</p><h2>Chọn một giờ để quan sát</h2></div><div class="legend"><span class="l-support">Nổi bật</span><span class="l-constraint">Lưu ý</span><span class="l-selected">Đang chọn</span></div></div><div class="b-hours">{#each hours as hour}<button class={hour.tone}><small>{hour.time}</small><strong>{hour.branch}</strong><i></i><span>{hour.note}</span></button>{/each}</div></section>
          <section class="b-focus"><div class="focus-clock"><span>09</span><i>:</i><span>42</span><small>GIỜ TỴ</small></div><div class="focus-copy"><p class="eyebrow">KẾT QUẢ Ở GIỜ ĐÃ CHỌN</p><h2>Không có tín hiệu bất thường ở lớp ẩn danh.</h2><p>Giờ Tỵ giữ trạng thái bình trong Tổng quan Ngày. Một Đánh giá cần mục đích; hồ sơ sinh là lớp bổ sung tùy chọn.</p><div class="reason-chips"><button>1 lý do</button><button>3 nguồn</button><button>Không có phân kỳ</button></div></div></section>
          <section class="b-pattern"><article><small>HỖ TRỢ 01</small><h3>Minh Đường hiện nhật</h3><p>Tín hiệu ngày Hoàng Đạo.</p></article><article><small>HỖ TRỢ 02</small><h3>Trực Thành</h3><p>Thế hoàn tất và định hình.</p></article><article class="warn"><small>GIỚI HẠN 01</small><h3>Mùi xung Sửu</h3><p>Chờ đối chiếu hồ sơ cá nhân.</p></article></section>
        </main>
        <aside class="b-inspector"><div class="inspector-head"><p>CHỨNG CỨ THEO TIÊU ĐIỂM</p><span>03 nguồn</span></div><div class="chain"><article><small>KẾT QUẢ</small><b>Giờ Tỵ · trạng thái bình</b></article><i>↓</i><article><small>LÝ DO</small><b>Không có tín hiệu giờ nổi trội</b></article><i>↓</i><article><small>CHỨNG CỨ</small><b>Bảng giờ Hoàng/Hắc Đạo</b><span>Đã duyệt · v1.10</span></article></div><div class="coverage-ring"><div><strong>67<small>%</small></strong></div><p><b>Độ phủ hiện tại</b><span>Không phải xác suất hay độ đúng.</span></p></div><div class="unknown-box"><small>CHƯA BIẾT</small><b>Ý định của bạn</b><p>Cần để tạo kết luận phù hợp theo việc.</p><button>Thêm mục đích →</button></div><button class="open-evidence">Mở toàn bộ chứng cứ ↗</button></aside>
      </div>
    </div>

  {:else}
    <div class="variant-c">
      <header class="c-header"><div><span class="seal">ÂL</span><div><b>Âm Lịch</b><small>Sổ quan sát mỗi ngày</small></div></div><nav><button class="active">Ngày</button><button>Tra cứu</button><button>Nguồn</button></nav><button class="c-profile">Ẩn danh · Thêm bối cảnh</button></header>
      <div class="c-datebar"><button>← 18.09</button><div><small>THỨ BẢY</small><strong>19</strong><span><b>THÁNG 09 · 2026</b>09 tháng 08 · ngày Ất Mùi</span></div><button>20.09 →</button><button class="today-button">Hôm nay</button></div>
      <main class="c-main">
        <section class="c-ledger">
          <div class="ledger-title"><div><p class="eyebrow">TÓM LƯỢC ẨN DANH</p><h1>Sổ Ngày</h1></div><span>Bạch Lộ<br/>Thu phân sau 3 ngày</span></div>
          <div class="ledger-row"><small>01 · HỖ TRỢ</small><div><h3>Minh Đường hiện nhật</h3><p>Một tín hiệu Hoàng Đạo được ghi nhận cho ngày.</p></div><button>2 nguồn →</button></div>
          <div class="ledger-row"><small>02 · HỖ TRỢ</small><div><h3>Trực Thành</h3><p>Ngày mang thế hoàn tất và định hình.</p></div><button>3 nguồn →</button></div>
          <div class="ledger-row warning"><small>03 · GIỚI HẠN</small><div><h3>Mùi xung Sửu</h3><p>Cần xét thêm nếu hồ sơ cá nhân có chi Sửu.</p></div><button>1 nguồn →</button></div>
          <div class="ledger-row unknown"><small>04 · CHƯA BIẾT</small><div><h3>Ngày có phù hợp việc của bạn?</h3><p>Chưa có mục đích nên chưa thể tạo Đánh giá Ngày.</p></div><button>Thêm mục đích +</button></div>
          <div class="ledger-note"><span>!</span><p><b>Đây không phải phán quyết tốt/xấu.</b> Sổ Ngày chỉ ghi các tín hiệu có nguồn và khoảng trống đang biết.</p></div>
        </section>
        <section class="c-hours-panel">
          <div class="c-panel-title"><div><p class="eyebrow">DÒNG THỜI GIAN · ĐỦ 12 GIỜ</p><h2>Sổ giờ</h2></div><div class="c-clock">09:42 <small>GIỜ TỴ</small></div></div>
          <div class="hour-table"><div class="hour-table-head"><span>Khoảng</span><span>Chi</span><span>Trạng thái ẩn danh</span><span>Chứng cứ</span></div>{#each hours as hour}<button class:current={hour.tone === 'selected'} class:marked={hour.tone === 'support'} class:warning={hour.tone === 'constraint'}><span>{hour.time}</span><strong>{hour.branch}</strong><span><i></i>{hour.note}</span><small>{hour.tone === 'support' ? '2 nguồn' : hour.tone === 'constraint' ? '1 lưu ý' : hour.tone === 'selected' ? 'đang mở' : '—'}</small></button>{/each}</div>
        </section>
        <aside class="c-evidence"><div class="c-evidence-head"><p class="eyebrow">NGĂN CHỨNG CỨ</p><h2>Giờ Tỵ</h2><span>09:00–11:00</span></div><div class="c-result"><small>KẾT QUẢ</small><b>Trạng thái bình ở lớp ẩn danh</b><p>Không có giờ nổi bật hay giới hạn đang áp dụng.</p></div><div class="c-source"><small>NGUỒN ĐANG DÙNG</small><b>Giờ Hoàng/Hắc Đạo</b><span>Đã duyệt · v1.10</span><button>Xem bản ghi nguồn ↗</button></div><div class="c-source"><small>CHƯA ÁP DỤNG</small><b>Ý định & hồ sơ</b><span>Thiếu dữ liệu · không tính là trung tính</span></div><div class="c-disclosure"><b>Độ phủ 2 / 4 lớp</b><p>Độ phủ mô tả dữ liệu hiện có, không phải độ chắc chắn.</p></div></aside>
      </main>
    </div>
  {/if}

  <PrototypeSwitcher current={variant} />
</div>

<style>
  :global(body) { margin: 0; }
  button { font: inherit; }
  .prototype-shell { min-height: 100%; width: 100%; overflow: auto; background: #f3f0e7; color: #20241f; }
  .prototype-flag { position: fixed; z-index: 90; top: 0; right: 24px; padding: 5px 10px; background: #9f3d2f; color: white; font: 9px/1.2 ui-monospace, monospace; letter-spacing: .12em; }
  .eyebrow { margin: 0 0 8px; color: #6d756a; font: 10px/1.3 ui-monospace, monospace; letter-spacing: .14em; }

  /* A — a calm, vertically guided narrative with progressive disclosure. */
  .variant-a { min-height: 100vh; background: #f6f2e8; color: #263027; font-family: Georgia, serif; }
  .a-header { position: sticky; z-index: 20; top: 0; display: grid; grid-template-columns: 220px 1fr 220px; align-items: center; min-height: 76px; padding: 0 32px; border-bottom: 1px solid #cfc9b8; background: rgba(246,242,232,.96); }
  .brand span { display: block; font: 700 15px/1.2 ui-monospace, monospace; letter-spacing: .18em; } .brand small { color: #70776d; }
  .date-stepper { display: flex; align-items: center; justify-content: center; gap: 20px; text-align: center; } .date-stepper b,.date-stepper small { display: block; } .date-stepper small { margin-top: 4px; color: #72796f; font: 11px ui-monospace, monospace; }
  .date-stepper button { border: 0; background: transparent; color: #566056; cursor: pointer; font-size: 26px; }
  .today { justify-self: end; border: 1px solid #a8afa4; border-radius: 999px; background: transparent; padding: 8px 13px; font: 11px ui-monospace, monospace; }
  .a-body { display: grid; grid-template-columns: minmax(0,1fr) 300px; max-width: 1320px; margin: 0 auto; }
  .a-main { padding: 54px 54px 120px; border-right: 1px solid #d6d0c1; }
  .a-intro { max-width: 820px; padding-bottom: 50px; } .a-intro h1 { max-width: 760px; margin: 0; font-size: clamp(34px,4vw,60px); font-weight: 400; line-height: 1.05; letter-spacing: -.035em; } .a-intro h1 em { color: #41644c; font-weight: 600; }
  .lead { max-width: 650px; margin: 22px 0; color: #5f675e; font: 16px/1.6 system-ui, sans-serif; }
  .primary { border: 0; border-radius: 2px; background: #263d2d; color: #fffdf5; padding: 13px 18px; cursor: pointer; font: 12px ui-monospace, monospace; }
  .pattern-section,.hours-section { padding: 42px 0; border-top: 1px solid #d6d0c1; }
  .section-heading { display: flex; gap: 18px; margin-bottom: 25px; } .section-heading>span { padding-top: 5px; color: #998c70; font: 11px ui-monospace, monospace; } .section-heading h2 { margin: 0; font-size: 26px; } .section-heading p { margin: 5px 0 0; color: #71776f; font: 12px system-ui, sans-serif; }
  .signal-grid { display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px; } .signal { min-height: 190px; padding: 18px; border: 1px solid #cbc5b6; background: rgba(255,255,255,.38); } .signal h3 { display: flex; justify-content: space-between; margin: 0 0 16px; font: 700 11px ui-monospace, monospace; letter-spacing: .12em; } .signal h3 span { opacity: .5; }.signal.support { border-top: 3px solid #52755c; }.signal.constraint { border-top: 3px solid #a7643f; }.signal.unknown { border-top: 3px solid #8c897f; }
  .signal-row { position: relative; display: block; width: 100%; padding: 11px 22px 11px 0; border: 0; border-top: 1px solid #ddd7ca; background: transparent; text-align: left; cursor: pointer; }.signal-row b,.signal-row small { display: block; }.signal-row b { font: 13px system-ui,sans-serif; }.signal-row small { margin-top: 5px; color: #71776f; font: 11px/1.35 system-ui,sans-serif; }.signal-row i { position:absolute;right:0;top:18px;font-style:normal; }
  .hour-ribbon { display: grid; grid-template-columns: repeat(12,minmax(56px,1fr)); border: 1px solid #cbc5b6; overflow-x: auto; }.hour-ribbon button { min-width: 62px; padding: 12px 4px; border: 0; border-right: 1px solid #d9d3c6; background: #f9f6ee; cursor:pointer; }.hour-ribbon small,.hour-ribbon b,.hour-ribbon i { display:block; }.hour-ribbon small { color:#787d75;font:9px ui-monospace,monospace; }.hour-ribbon b { margin:8px 0;font-size:16px; }.hour-ribbon i { min-height: 20px;color:#777;font:8px/1.2 ui-monospace,monospace;font-style:normal; }.hour-ribbon button.notable { box-shadow: inset 0 3px #52755c; }.hour-ribbon button.caution { box-shadow: inset 0 3px #a7643f; }.hour-ribbon button.active { background:#263d2d;color:white; }.hour-ribbon button.active small,.hour-ribbon button.active i { color:#dce6db; }
  .selected-hour { display:grid;grid-template-columns:140px 1fr auto;gap:22px;align-items:center;margin-top:12px;padding:20px;border:1px solid #cbc5b6;background:#ede8db; }.time-mark small,.time-mark strong,.time-mark span { display:block; }.time-mark small{font:9px ui-monospace,monospace;color:#777}.time-mark strong{margin:5px 0;font-size:28px}.time-mark span{font:10px ui-monospace,monospace}.selected-hour h3{margin:0;font-size:16px}.selected-hour p{margin:5px 0 0;color:#666;font:12px/1.45 system-ui,sans-serif}.selected-hour button{border:0;background:transparent;color:#35543d;font:11px ui-monospace,monospace;cursor:pointer}
  .a-aside { padding: 54px 26px 120px; background:#eee9dc; }.a-aside section { padding:0 0 28px;margin-bottom:28px;border-bottom:1px solid #d1caba; }.identity { display:flex;align-items:baseline;gap:8px; }.identity strong{font-size:36px;font-weight:400}.identity span{color:#777}dl{margin:18px 0 0}dl>div{display:flex;justify-content:space-between;padding:8px 0;border-top:1px solid #d8d1c2;font:11px system-ui,sans-serif}dt{color:#777}dd{margin:0;font-weight:600}.aside-title{display:flex;justify-content:space-between;align-items:center}.aside-title h3{font-size:15px}.aside-title span{color:#9c5b37;font:9px ui-monospace,monospace}.coverage{margin:15px 0}.coverage>div:first-child{display:flex;justify-content:space-between;font:10px system-ui,sans-serif}.coverage small{color:#747a72}.bar{height:3px;margin-top:7px;background:#d2ccbd}.bar i{display:block;height:100%;background:#52755c}.bar i.missing{background:#a29a88}.text-button{padding:0;border:0;background:transparent;color:#35543d;font:10px ui-monospace,monospace}.context-card button{display:block;width:100%;margin-top:7px;padding:9px;border:1px solid #bdb6a6;background:transparent;text-align:left;font:10px ui-monospace,monospace}.context-card h3{margin:0}.context-card>p:not(.eyebrow){color:#6c736b;font:11px/1.4 system-ui,sans-serif}

  /* B — an instrument panel where the selected hour is the spatial anchor. */
  .variant-b { min-height:100vh;background:#18201c;color:#e9eee7;font-family:system-ui,sans-serif; }.b-header{display:grid;grid-template-columns:220px 1fr 220px;align-items:center;height:78px;padding:0 24px;border-bottom:1px solid #3a463e;background:#111713}.b-brand{font:700 22px ui-monospace,monospace;color:#eabf64}.b-brand span{display:block;color:#849087;font-size:8px;letter-spacing:.25em}.b-date{display:flex;justify-content:center;align-items:center;gap:18px;text-align:center}.b-date button{border:1px solid #3d4941;background:transparent;color:#cbd2cb;width:30px;height:30px}.b-date small,.b-date b,.b-date span{display:block}.b-date small{font:8px ui-monospace,monospace;color:#879087;letter-spacing:.12em}.b-date b{font:18px ui-monospace,monospace;letter-spacing:.06em}.b-date span{color:#97a097;font-size:10px}.b-now{justify-self:end;font:10px ui-monospace,monospace;color:#aab3ab}.b-now i{display:inline-block;width:7px;height:7px;margin-right:6px;border-radius:50%;background:#eabf64;box-shadow:0 0 10px #eabf64}.b-grid{display:grid;grid-template-columns:190px minmax(600px,1fr) 280px;min-height:calc(100vh - 78px)}
  .b-nav{display:flex;flex-direction:column;border-right:1px solid #39453d;background:#141b17;padding:28px 12px 90px}.b-nav>p{margin:0 10px 15px;color:#748078;font:8px ui-monospace,monospace;letter-spacing:.18em}.b-nav>button{position:relative;display:grid;grid-template-columns:28px 1fr;gap:6px;padding:12px 9px;border:0;border-radius:3px;background:transparent;color:#96a198;text-align:left;font-size:11px}.b-nav>button span{color:#67726a;font:9px ui-monospace,monospace}.b-nav>button.active{background:#2d3a32;color:#fff;box-shadow:inset 2px 0 #eabf64}.b-nav>button i{position:absolute;right:8px;font:8px ui-monospace,monospace;color:#eabf64}.b-profile{margin-top:auto;padding:14px;border:1px solid #39453d}.b-profile small,.b-profile b{display:block}.b-profile small{color:#77827a;font:8px ui-monospace,monospace}.b-profile b{margin:5px 0 12px;font-size:12px}.b-profile button{padding:0;border:0;background:transparent;color:#eabf64;font:9px ui-monospace,monospace}
  .b-stage{padding:28px 28px 110px;overflow:hidden}.b-summary{display:flex;align-items:center;justify-content:space-between;padding-bottom:25px}.b-summary h1{margin:0;font-size:22px;font-weight:500}.b-summary h1 span{color:#59645d}.b-summary button{padding:10px 13px;border:1px solid #d1a84f;background:#eabf64;color:#1b211d;font:9px ui-monospace,monospace}.b-timeline{padding:20px;border:1px solid #3b493f;background:#202a24}.timeline-head{display:flex;justify-content:space-between;align-items:flex-start}.timeline-head h2{margin:0;font-size:15px}.legend{display:flex;gap:13px;color:#879188;font:8px ui-monospace,monospace}.legend span:before{content:'';display:inline-block;width:6px;height:6px;margin-right:5px;border-radius:50%;background:#777}.legend .l-support:before{background:#74a684}.legend .l-constraint:before{background:#cb7755}.legend .l-selected:before{background:#eabf64}.b-hours{display:grid;grid-template-columns:repeat(12,1fr);margin-top:20px}.b-hours button{position:relative;min-width:47px;height:110px;padding:8px 2px;border:0;border-right:1px solid #39453d;background:transparent;color:#96a098;text-align:center}.b-hours small,.b-hours strong,.b-hours span{display:block}.b-hours small{font:7px ui-monospace,monospace}.b-hours strong{margin-top:9px;font-size:14px}.b-hours i{display:block;width:7px;height:7px;margin:12px auto;border:1px solid #667168;border-radius:50%}.b-hours span{font:7px/1.2 ui-monospace,monospace}.b-hours button.support i{background:#74a684;border-color:#74a684}.b-hours button.constraint i{background:#cb7755;border-color:#cb7755}.b-hours button.selected{z-index:2;background:#eabf64;color:#18201c;box-shadow:0 6px 20px rgba(0,0,0,.25)}.b-hours button.selected i{background:#18201c;border-color:#18201c}.b-focus{display:grid;grid-template-columns:150px 1fr;gap:30px;padding:28px 0;border-bottom:1px solid #39453d}.focus-clock{display:grid;grid-template-columns:1fr 8px 1fr;align-items:center}.focus-clock span{font:34px ui-monospace,monospace}.focus-clock i{font-style:normal;color:#eabf64}.focus-clock small{grid-column:1/4;color:#7e8980;font:8px ui-monospace,monospace;letter-spacing:.15em}.focus-copy h2{margin:0;font-size:20px;font-weight:500}.focus-copy>p:not(.eyebrow){max-width:660px;color:#9fa9a1;font-size:12px;line-height:1.5}.reason-chips{display:flex;gap:6px}.reason-chips button{padding:5px 8px;border:1px solid #465249;border-radius:999px;background:transparent;color:#aab4ab;font:8px ui-monospace,monospace}.b-pattern{display:grid;grid-template-columns:repeat(3,1fr);gap:8px;padding-top:18px}.b-pattern article{padding:13px;border-left:2px solid #74a684;background:#202a24}.b-pattern article.warn{border-color:#cb7755}.b-pattern small{color:#78847b;font:7px ui-monospace,monospace}.b-pattern h3{margin:8px 0 3px;font-size:12px}.b-pattern p{margin:0;color:#8d978f;font-size:9px}
  .b-inspector{padding:28px 18px 100px;border-left:1px solid #39453d;background:#151c18}.inspector-head{display:flex;justify-content:space-between;padding-bottom:15px;border-bottom:1px solid #39453d;color:#7f8a82;font:8px ui-monospace,monospace}.chain{padding:20px 0}.chain article{padding:12px;border:1px solid #37433b;background:#1c2520}.chain article small,.chain article b,.chain article span{display:block}.chain article small{color:#758078;font:7px ui-monospace,monospace}.chain article b{margin-top:6px;font-size:10px}.chain article span{margin-top:4px;color:#859088;font-size:8px}.chain>i{display:block;margin:4px 0 4px 13px;color:#69736c;font-style:normal}.coverage-ring{display:flex;align-items:center;gap:13px;padding:16px 0;border-top:1px solid #39453d;border-bottom:1px solid #39453d}.coverage-ring>div{display:grid;place-items:center;width:58px;height:58px;border:5px solid #65746a;border-right-color:#27322b;border-radius:50%}.coverage-ring strong{font:16px ui-monospace,monospace}.coverage-ring strong small{font-size:8px}.coverage-ring p b,.coverage-ring p span{display:block}.coverage-ring p b{font-size:10px}.coverage-ring p span{margin-top:4px;color:#77837a;font-size:8px}.unknown-box{margin-top:16px;padding:14px;background:#2b2b23;border:1px solid #53503b}.unknown-box small,.unknown-box b{display:block}.unknown-box small{color:#d7b45e;font:7px ui-monospace,monospace}.unknown-box b{margin-top:7px;font-size:11px}.unknown-box p{color:#a7a591;font-size:9px}.unknown-box button,.open-evidence{padding:0;border:0;background:transparent;color:#eabf64;font:8px ui-monospace,monospace}.open-evidence{margin-top:22px;width:100%;padding:10px;border:1px solid #4a574e}

  /* C — a dense chronological ledger with evidence always adjacent. */
  .variant-c{min-height:100vh;background:#f8f7f2;color:#24251f;font-family:Arial,sans-serif}.c-header{display:flex;align-items:center;justify-content:space-between;height:62px;padding:0 26px;border-bottom:1px solid #c8c5ba;background:#fff}.c-header>div{display:flex;align-items:center;gap:10px}.seal{display:grid;place-items:center;width:34px;height:34px;background:#963c30;color:#fff;font:700 14px Georgia,serif}.c-header b,.c-header small{display:block}.c-header b{font-family:Georgia,serif}.c-header small{color:#7c7d76;font-size:9px}.c-header nav{height:100%}.c-header nav button{height:100%;padding:0 16px;border:0;border-bottom:2px solid transparent;background:transparent;font-size:11px}.c-header nav button.active{border-color:#963c30}.c-profile{padding:7px 10px;border:1px solid #bdbab0;background:#fff;font-size:9px}.c-datebar{display:flex;align-items:center;gap:18px;min-height:96px;padding:0 26px;border-bottom:1px solid #c8c5ba;background:#eeece4}.c-datebar>button{border:0;background:transparent;color:#696b64;font:10px ui-monospace,monospace}.c-datebar>div{display:grid;grid-template-columns:28px 55px 1fr;align-items:center;min-width:330px}.c-datebar>div>small{font:8px ui-monospace,monospace;writing-mode:vertical-rl}.c-datebar strong{font:44px Georgia,serif;line-height:1}.c-datebar span b,.c-datebar span{display:block}.c-datebar span{color:#676961;font-size:9px}.c-datebar span b{margin-bottom:5px;color:#272923;font:11px ui-monospace,monospace}.c-datebar .today-button{margin-left:auto;padding:8px 12px;border:1px solid #aaa79e;background:#f8f7f2;color:#262821}.c-main{display:grid;grid-template-columns:minmax(360px,.9fr) minmax(430px,1.15fr) 280px;min-height:calc(100vh - 159px)}.c-ledger,.c-hours-panel,.c-evidence{padding:28px 24px 110px}.c-ledger{border-right:1px solid #cbc8bd}.ledger-title,.c-panel-title{display:flex;justify-content:space-between;align-items:flex-start;padding-bottom:18px;border-bottom:2px solid #292a25}.ledger-title h1,.c-panel-title h2{margin:0;font:28px Georgia,serif}.ledger-title>span{color:#6f716a;text-align:right;font:9px/1.5 ui-monospace,monospace}.ledger-row{display:grid;grid-template-columns:90px 1fr auto;gap:12px;padding:16px 0;border-bottom:1px solid #d5d2c8}.ledger-row>small{color:#48705a;font:8px ui-monospace,monospace}.ledger-row.warning>small{color:#a35b3f}.ledger-row.unknown>small{color:#77736a}.ledger-row h3{margin:0;font:15px Georgia,serif}.ledger-row p{margin:5px 0 0;color:#71736c;font-size:10px;line-height:1.4}.ledger-row button{align-self:start;padding:0;border:0;background:transparent;color:#5c655e;font:8px ui-monospace,monospace}.ledger-note{display:flex;gap:11px;margin-top:18px;padding:12px;background:#ebe7dc}.ledger-note span{font:700 15px Georgia,serif}.ledger-note p{margin:0;color:#65675f;font-size:9px;line-height:1.5}.ledger-note b{color:#272923}.c-hours-panel{border-right:1px solid #cbc8bd;background:#f1efe8}.c-panel-title{align-items:end}.c-clock{text-align:right;font:18px ui-monospace,monospace}.c-clock small{display:block;color:#75776f;font-size:8px}.hour-table{margin-top:10px}.hour-table-head,.hour-table button{display:grid;grid-template-columns:60px 45px 1fr 60px;align-items:center;width:100%;text-align:left}.hour-table-head{padding:8px 10px;color:#85877f;font:7px ui-monospace,monospace}.hour-table button{min-height:40px;padding:7px 10px;border:0;border-top:1px solid #d4d1c7;background:transparent;color:#4b4d46;font-size:9px}.hour-table button strong{font:15px Georgia,serif}.hour-table button>span:nth-child(3){font-size:9px}.hour-table button>span i{display:inline-block;width:5px;height:5px;margin-right:6px;border-radius:50%;background:#aaa}.hour-table button.marked>span i{background:#4e765f}.hour-table button.warning>span i{background:#a55c40}.hour-table button.current{background:#292f29;color:white;box-shadow:4px 4px 0 #b7a36a}.hour-table button small{text-align:right;color:inherit;font:7px ui-monospace,monospace}.c-evidence{background:#fff}.c-evidence-head{padding-bottom:16px;border-bottom:2px solid #292a25}.c-evidence-head h2{margin:0;font:24px Georgia,serif}.c-evidence-head span{color:#6f716a;font:9px ui-monospace,monospace}.c-result,.c-source,.c-disclosure{padding:17px 0;border-bottom:1px solid #d6d3c9}.c-result small,.c-result b,.c-source small,.c-source b,.c-source span{display:block}.c-result small,.c-source small{color:#777970;font:7px ui-monospace,monospace;letter-spacing:.1em}.c-result b,.c-source b{margin-top:7px;font:13px Georgia,serif}.c-result p,.c-disclosure p{color:#71736c;font-size:9px;line-height:1.5}.c-source span{margin-top:5px;color:#74766f;font-size:8px}.c-source button{margin-top:12px;padding:0;border:0;background:transparent;color:#426351;font:8px ui-monospace,monospace}.c-disclosure{margin-top:18px;padding:14px;background:#eeeae0}.c-disclosure b{font-size:10px}.c-disclosure p{margin-bottom:0}

  @media (max-width: 1050px) {
    .a-body { grid-template-columns: 1fr; }.a-aside { display:grid;grid-template-columns:repeat(3,1fr);gap:20px }.a-main{border-right:0}.signal-grid{grid-template-columns:1fr}.b-grid{grid-template-columns:150px minmax(620px,1fr)}.b-inspector{display:none}.c-main{grid-template-columns:1fr 1fr}.c-evidence{display:none}
  }
</style>
