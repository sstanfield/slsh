---
title: FAQ layout
permalink: mydoc_faq_layout.html
sidebar: mydoc_sidebar
tags: [special_layouts]
keywords: frequently asked questions, FAQ, question and answer, collapsible sections, expand, collapse
last_updated: November 30, 2015
toc: false
folder: mydoc
---

<div class="panel-group" id="accordion">
    <div class="panel panel-default">
        <div class="panel-heading">
            <h4 class="panel-title">
                <a class="noCrossRef accordion-toggle" data-toggle="collapse" data-parent="#accordion" href="#collapseOne">Will sl-sh be supported on windows?</a>
            </h4>
        </div>
        <div id="collapseOne" class="panel-collapse collapse noCrossRef">
            <div class="panel-body">
            Work is being done in <a href="https://github.com/sl-sh-dev/sl-liner">sl-liner</a> and <a href="https://github.com/sl-sh-dev/sl-console">sl-console</a> to make this possible but currently it is not supported.
            </div>
        </div>
    </div>
    <!-- /.panel -->
    <div class="panel panel-default">
        <div class="panel-heading">
            <h4 class="panel-title">
                <a class="noCrossRef accordion-toggle" data-toggle="collapse" data-parent="#accordion" href="#collapseTwo">Is it any good?</a>
            </h4>
        </div>
        <div id="collapseTwo" class="panel-collapse collapse noCrossRef">
            <div class="panel-body">
            <a href="https://news.ycombinator.com/item?id=3067434">Yes.</a>
            </div>
        </div>
    </div>
    <div class="panel panel-default">
        <div class="panel-heading">
            <h4 class="panel-title">
                <a class="noCrossRef accordion-toggle" data-toggle="collapse" data-parent="#accordion" href="#collapseThree">Wouldn't sl-sh run faster with a VM?</a>
            </h4>
        </div>
        <div id="collapseThree" class="panel-collapse collapse noCrossRef">
            <div class="panel-body">
            Yes! One is currently being developed. It is called <a href="https://github.com/sstanfield/slvm">slvm</a>
            </div>
        </div>
    </div>
</div>
<!-- /.panel-group -->

{% include links.html %}
