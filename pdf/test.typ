#import "lib.typ": *

#show: resume_template( my_title("csy", 2013, 2200) )

== 基本信息

#my_info( 
  id: 100 ,
  sex:  1 ,
  title: 1 
)

== 科研

#my_paper(
  name: "csy paper",
  year: 2024,
  src: "csy committee",
  rank: 1,
  comm: true,
  lvl: 3,
  ty: 2
)

== 项目

#my_project(
  name: "CSY project",
  start: 2024,
  end: none,
  rank: 2,
  fund: 100000,
  src: "CSY committee",
  secret_lvl: 1,
  ty: 1
)

== 课程

#my_course(
  name: "CSY Research",
  hour: 200,
  year: 2024,
  semester: 2
)