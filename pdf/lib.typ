#import "@preview/basic-resume:0.2.8": *

#let resume_template(name) = {
  let name = name

  resume.with(
    author: name,
    
    accent-color: "#26428b",
    font: "New Computer Modern",
    paper: "us-letter",
    author-position: center,
    personal-info-position: left,
  )
}

#let map_title = ( "", "博士后", "助教", "讲师", "副教授", "特任教授", "教授", "助理研究员", "特任副研究员", "副研究员", "特任研究员", "研究员")

#let map_sex = ( "", "男", "女" )

#let map_semster = ( "", "春", "夏","秋")

#let my_title(name, from, to) = {
  if from != none and to != none {
    name + " ( " + str(from) + "-" + str(to) + " ) "
  } else if from != none {
    name + " ( " + str(from) + "- ) "
  } else if to != none {
    name + " ( -" + str(to) + " ) "
  } else {
    name + "( 生涯 )"
  }
}

#let my_info(id: "", sex: "", title: "") = {
  grid(columns: (1fr, 1fr, 1fr))[ 工号： #id ][ 性别： #map_sex.at(sex) ][ 职称： #map_title.at(title) ]
}

#let map_paperlvl = ("", "CCF-A", "CCF-B", "CCF-C", "中文CCF-A", "中文CCF-B", "无级别")

#let map_paperty = ("", "full paper", "short paper", "poster paper", "demo paper")

#let my_paper(name: "", year: "", src: "", rank: "", comm: false, lvl: "", ty: "" ) = {
  project(
    dates: year,
    name: "排名" + str(rank),
    role: name,
    url: ""
  )
  [
    #if comm {
      [- 通讯作者]
    }
    - 论文级别：#map_paperlvl.at(lvl)
    - 论文类型：#map_paperty.at(ty)
  ]
}

#let my_course(name: "", hour: "", year: "", semester: "") = {
  grid(columns: (1fr, 1fr, 1fr))[ 课程名： #name ][ 主讲学时： #hour ][ 学期： #year #map_semster.at(semester) ]
}

#let map_projectty = ("", "国家级项目", "省部级项目", "市厅级项目", "企业合作项目", "其它类型项目")

#let my_project(name: "", start: "", end: "", rank: "", fund: "", src: "", secret_lvl: "", ty: "") = {
  let date = str(start) + "-"
  let date = date + if end == none {""} else {str(end)}  
  project(
    dates: date,
    name: "排名" + str(rank),
    role: name,
    url: ""
  )
  [
    - 经费：#fund
    - 项目来源: #src
    - 保密等级: #if secret_lvl == 1 [不保密] else [保密]
    - 项目类型：#map_projectty.at(ty)
  ]
}