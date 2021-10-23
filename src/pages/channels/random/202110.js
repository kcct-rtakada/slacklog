import React from "react"
import "../../../styles/message.css"
import Layout from "../../../components/layout"

export default function random202110() {
  return (
    <Layout>

      <h4 className={"title-h4 border-bottom"}>#random / 2021年10月</h4>

      <div className={"border-bottom mt-16 fs-14"}>
        <div className={"float-left mr-8"}>
          <img className={"round-img"} width={"36"} height={"36"} alt={""}
               src={"https://avatars.slack-edge.com/2021-04-06/1931380408275_62c83b37e4d6201b7fd5_72.jpg"}/>
        </div>
        <div>
          <span className={"text-bold mr-4"}>髙田崚介</span>
          <span>8日 15:49:56</span>
        </div>
        <div className={"overflow-hidden mb-16"}>
          <div className={"pt-4 pb-4"}>
            最近、ファイル共有サイトのAresを使って資料収集とかに使った人いますか？
            <br/>
            ファイル共有や内容自体に問題があるというわけではなく、
            <br/>
            割と大きな通信が発生したので、それがどこで発生したのか確認依頼が電子科に来ました。
            <br/>
            心当たりがある人はDMとかでも良いので教えてくれると助かります。
          </div>
        </div>
      </div>

      <div className={"border-bottom mt-16 fs-14"}>
        <div className={"float-left mr-8"}>
          <img className={"round-img"} width={"36"} height={"36"} alt={""}
               src={"https://avatars.slack-edge.com/2021-04-06/1931380408275_62c83b37e4d6201b7fd5_72.jpg"}/>
        </div>
        <div>
          <span className={"text-bold mr-4"}>髙田崚介</span>
          <span>13日 11:37:36</span>
        </div>
        <div className={"overflow-hidden mb-16"}>
          <div className={"pt-4 pb-4"}>
            本日、お腹の調子がよくなくて午前はお休みとりましたが、午後には出勤予定です！
            <br/>
            13時～で面談よろしくおねがいします。
          </div>
        </div>
      </div>

    </Layout>
  )
}
