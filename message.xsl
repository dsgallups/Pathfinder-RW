<!-- 
 Copyright 1995-2012 Ellucian Company L.P. and its affiliates. 
 $Id$ -->
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform"> 
<!-- Purdue modification history
  8/2017 C. Karle Add s to Disclaimer 
-->

<!-- Variables available for customizing -->
<xsl:variable name="LabelProgressBar">    Degree Progress </xsl:variable>
<xsl:variable name="LabelStillNeeded">    Still Needed:  </xsl:variable>
<xsl:variable name="LabelAlertsReminders">  Alerts and Reminders </xsl:variable>
<xsl:variable name="LabelFallthrough">      Fallthrough Courses </xsl:variable>
<xsl:variable name="LabelInprogress">       In-progress </xsl:variable>
<xsl:variable name="LabelOTL">              Not Counted </xsl:variable>
<!-- Purdue Localization
<xsl:variable name="LabelInsufficient">     Insufficient </xsl:variable>-->
<xsl:variable name="LabelInsufficient">     Insufficient Grades </xsl:variable>

<xsl:variable name="LabelSplitCredits">     Split Credits </xsl:variable>
<xsl:variable name="LabelPlaceholders">     Planner Placeholders </xsl:variable>
<xsl:variable name="LabelIncludedBlocks">   Blocks included in this block</xsl:variable>
<xsl:variable name="vShowTitleCreditsInHint">Y</xsl:variable>
<xsl:variable name="vLabelSchool"     >Level</xsl:variable>
<xsl:variable name="vLabelDegree"     >Degree</xsl:variable>
<xsl:variable name="vLabelMajor"      >Major</xsl:variable>
<xsl:variable name="vLabelMinor"      >Minor</xsl:variable>
<xsl:variable name="vLabelCollege"    >College</xsl:variable>
<xsl:variable name="vLabelLevel"      >Classification</xsl:variable>
<xsl:variable name="vLabelAdvisor"    >Advisor</xsl:variable>
<xsl:variable name="vLabelStudentID"  >ID</xsl:variable>
<xsl:variable name="vLabelStudentName">Student</xsl:variable>
<xsl:variable name="vLabelOverallGPA" >Overall GPA</xsl:variable>
<xsl:variable name="vGetCourseInfoFromServer">Y</xsl:variable>
<xsl:variable name="vCreditDecimals">0.###</xsl:variable>
<xsl:variable name="vProgressBarPercent">Y</xsl:variable>
<!-- Purdue Localization Change Credits Progress Bar to not display -->
<xsl:variable name="vProgressBarCredits">N</xsl:variable>
<xsl:variable name="vProgressBarRulesText">Percentage of requirements completed</xsl:variable>
<xsl:variable name="vProgressBarCreditsText">Percentage of credits completed</xsl:variable>
<xsl:variable name="vShowPetitions">N</xsl:variable>
<xsl:variable name="vShowToInsteadOfColon">Y</xsl:variable> <!-- ":" is replaced with " to " in classes/credits range -->

<xsl:key name="XptKey"    match="Audit/ExceptionList/Exception" use="@Id_num"/>
<xsl:key name="ClsKey"    match="Audit/Clsinfo/Class" use="@Id_num"/>
<xsl:key name="NonKey"    match="Audit/Clsinfo/Noncourse" use="@Id_num"/>
<xsl:key name="BlockKey"  match="Audit/Block" use="@Req_id"/>
<xsl:key name="BlockKey2" match="Audit/Block" use="@Req_type"/>

<xsl:template match="Audit">
<html>
   <xsl:call-template name="tTitleForPrintedAudit"/>
   <link rel="stylesheet" href="DashboardStyles.css" type="text/css" /><link rel="stylesheet" href="DashboardLocalizedStyles.css" type="text/css" />
<script id="auditFormJs" language="JavaScript" type="text/javascript">
//////////////////////////////////////////////////////////////////////
// Get the information for this course when the user clicks on the course
// as part of AdviceLink
//////////////////////////////////////////////////////////////////////
function GetCourseInfo(sDisc, sNumber, sAttribute, sAttributeOp)
{
var sWindowParams = "width=600,height=300,toolbar=no,location=no,directories=no,status=no,menubar=no,scrollbars=yes,resizable=yes";
var sWindowName = "wCourseInfo";
var wNew = window.open("", sWindowName, sWindowParams);

<xsl:choose>
  <xsl:when test="normalize-space(/Report/@rptCourseLeafURL)!=''">
   <!-- http://sungard.dev8.leepfrog.com/courseinfo/?list=ENGL:103&major=MATH,HIST&stylesheet=CourseLeaf.xsl -->
   sHref = "<xsl:copy-of select="normalize-space(/Report/@rptCourseLeafURL)" />" + "?";
   sHref = sHref + "list=" + sDisc + "-" + sNumber;
   <!-- Add the list of majors   -->
   sHref = sHref + "&amp;major=";
   <xsl:for-each select="/Report/Audit/Deginfo/Goal[@Code='MAJOR']">
   sHref = sHref + "<xsl:value-of select="normalize-space(@Value)" />"; 
    <xsl:if test="position()!=last()">sHref = sHref + ",";</xsl:if>  <!-- add a comma -->
   </xsl:for-each>
    // Stylesheet needs to be stored on CourseLeaf server (not on DW server)
   sHref = sHref + "&amp;stylesheet=DegreeWorks-CourseLink-Description.xsl";
  //alert ("GetCourseInfo - CourseLeaf Href = " + sHref);
  wNew.location.href = sHref;
  </xsl:when>
  <xsl:otherwise>
   sThisAttribute = "";
   sThisAttributeOp = "";
   if (sAttribute != undefined)
   {
      sThisAttribute = sAttribute;
       if (sAttributeOp != undefined)
       {
        if (sAttributeOp == "&lt;&gt;")
         sThisAttributeOp = "&lt;&gt;";    // not-equals
        else if (sAttributeOp == "&lt;=")
         sThisAttributeOp = "&lt;=";       // less-than-equals
        else if (sAttributeOp == "&gt;=")
         sThisAttributeOp = "&gt;=";       // greater-than-equals
        else if (sAttributeOp == "&lt;")
         sThisAttributeOp = "&lt;";        // less-than
        else if (sAttributeOp == "&gt;")
         sThisAttributeOp = "&gt;";        //   greater-than
          else // default to equals
         sThisAttributeOp = "=";           // equsls
       }
   }

    // Do not use the frmCourseInfo form in frControl because SEP cannot make use of it
    // Instead both in classic and in SEP we will create a new form that we will use instead

    var method = "POST";
    // This --path-- will be replaced by a complete URL to degreeworks by the SEP Java code
    // If we are not w/in SEP then the path needs to be the name we find in frControl
    var pathToCgi = '--path--';

    // If we are not in SEP the frControl frame should exist - so get the action
    if (pathToCgi.substring(0,2) == "--")
      pathToCgi = top.frControl.document.frmCourseInfo.action;

    // Create a new form for the request
    var form = document.createElement("form");
    // Move the submit function to another variable so that it doesn't get overwritten
    form._submit_function_ = form.submit;

    form.setAttribute("method", method);
    form.setAttribute("action", pathToCgi);
    form.setAttribute("target", sWindowName);
    
    appendFormChild(form, "COURSEDISC",   '"' + sDisc   + '"');
    appendFormChild(form, "COURSENUMB",   '"' + sNumber + '"');
    appendFormChild(form, "SCRIPT",       "SD2COURSEINFO");
    appendFormChild(form, "COURSEATTR",   sThisAttribute);
    appendFormChild(form, "COURSEATTROP", sThisAttributeOp);
    appendFormChild(form, "REPORT",        '<xsl:value-of select="/Report/@rptReportCode" />');
    appendFormChild(form, "REPORTUCLASS",  '<xsl:value-of select="/Report/@rptReportCode" /><xsl:value-of select="/Report/@rptUserClass" />');
    
    appendFormChild(form, "ContentType", "xml");
    
    document.body.appendChild(form);
    form._submit_function_(); // call the renamed function
    wNew.focus(); <!--  needed in case window was already open -->
  </xsl:otherwise>
</xsl:choose>
}

//////////////////////////////////////////////////////////////////////
function appendFormChild(form, inputName, inputValue)
{
  var hiddenElement = document.createElement("input");
  hiddenElement.setAttribute("type", "hidden");
  hiddenElement.setAttribute("name", inputName);
  hiddenElement.setAttribute("value",inputValue);
 
  form.appendChild(hiddenElement);
}

//////////////////////////////////////////////////////////////////////
// This is setup to link to LeepFrog's InstantRapport chat system.
// This can be modified to work with other systems however.
//////////////////////////////////////////////////////////////////////
function LinkToAdvisorChat(sAdvisorId)
{
var sWindowParams = "width=400,height=400,toolbar=no,location=no,directories=no,status=no,menubar=no,scrollbars=yes,resizable=yes";
var sWindowName = "wAdvisorChat";
var wNew = window.open("", sWindowName, sWindowParams);
//wNew.resizeTo (iWidth, iHeight);
//wNew.moveTo   (iXPlace, iYPlace);

//alert ("LinkToAdvisorChat: " advisor-id=" + sAdvisorId);

sProfileInfo="xxxx";
// MAJORS
<xsl:for-each select="/Report/Audit/Deginfo/Goal[@Code='MAJOR']">
  sProfileInfo= sProfileInfo + "&amp;MAJOR" + "<xsl:value-of select="position()"/>" + "=" + "<xsl:value-of select="@ValueLit" />";
</xsl:for-each>

sProfileInfo= sProfileInfo + "&amp;MAJORLABEL=" + "<xsl:copy-of select="$vLabelMajor"/><xsl:if test="count(/Report/Audit/Deginfo/Goal[@Code='MAJOR'])>1">s</xsl:if>";
sProfileInfo= sProfileInfo + "&amp;CREDITSLABEL=" + "<xsl:copy-of select="normalize-space(/Report/@rptCreditsLiteral)"/>";

// ADVISORS
<xsl:for-each select="/Report/Audit/Deginfo/Goal[@Code='ADVISOR']">
  sProfileInfo= sProfileInfo + "&amp;ADVISOR<xsl:value-of select="position()"/>=<xsl:value-of select="@Advisor_name" />";
</xsl:for-each>
// LEVEL
sProfileInfo= sProfileInfo + "&amp;LEVEL=<xsl:value-of select="/Report/Audit/Deginfo/DegreeData/@Stu_levelLit" />";
// DEGREE
sProfileInfo= sProfileInfo + "&amp;DEGREE=<xsl:value-of select="/Report/Audit/Deginfo/DegreeData/@DegreeLit" />";
sProfileInfo= sProfileInfo + "&amp;DEGREECODE=<xsl:value-of select="/Report/Audit/Deginfo/DegreeData/@Degree" />";
// SCHOOL
sProfileInfo= sProfileInfo + "&amp;SCHOOLCODE=<xsl:value-of select="/Report/Audit/Deginfo/DegreeData/@School" />";
// GPA
sProfileInfo= sProfileInfo + "&amp;GPA=<xsl:value-of select="/Report/Audit/AuditHeader/@SSGPA" />";
// CATALOG YEAR
sProfileInfo= sProfileInfo + "&amp;CATYR=<xsl:value-of select="/Report/Audit/Block/@Cat_yrLit" />";
// CLASSES APPLIED
sProfileInfo= sProfileInfo + "&amp;CLASSES=<xsl:value-of select="/Report/Audit/Block/@Classes_applied" />";
// CREDITS APPLIED
sProfileInfo= sProfileInfo + "&amp;CREDITS=<xsl:value-of select="/Report/Audit/Block/@Credits_applied" />";
// EMAIL ADDRESS
sProfileInfo= sProfileInfo + "&amp;EMAIL=<xsl:value-of select="/Report/Audit/AuditHeader/@Stu_email" />";

// The control frame has the form to submit
var frm = top.frControl.document.frmAdvisorChat; 
if (frm == "undefined") alert ("frmAdvisorChat was not found in control window");      
frm.target = sWindowName;
frm.ADVISORID.value   = sAdvisorId;
frm.PROFILEINFO.value = sProfileInfo; //+ '"'; // surround in quotes
frm.submit();
wNew.focus(); // needed in case window was already open
} // linktoadvisorchat

//////////////////////////////////////////////////////////////////////
// Update the audit with the status and description.
//////////////////////////////////////////////////////////////////////
function UpdateAudit()                                          
{
if (top.frControl == null) {alert ("frControl not defined - can't UpdateAudit"); return;}
//alert ("UpdateAudit enter");
var frmThis = document.frmAudit;
//  The control frame has the form to submit 
var frmSubmit = top.frControl.document.frmUpdateAudit;       
if (typeof(frmThis.selFreeze) != "undefined")
  frmSubmit.FREEZETYPE.value =  frmThis.selFreeze.options[frmThis.selFreeze.selectedIndex].value
if (typeof(frmThis.auditdescription) != "undefined")
  frmSubmit.AUDITDESC.value = '"' + frmThis.auditdescription.value + '"'; // put in dbl-quotes in case user entered a dbl-quote or ampersand
frmSubmit.AUDITID.value =  '<xsl:value-of select="/Report/Audit/AuditHeader/@Audit_id" />';
frmSubmit.STUID.value   =  '<xsl:value-of select="/Report/Audit/AuditHeader/@Stu_id" />';
frmSubmit.USERID.value  =  '<xsl:value-of select="/Report/@rptUsersId" />';
//alert ("UpdateAudit submit");
frmSubmit.target = "frHold"; // the hidden frame
frmSubmit.submit();
} // updateaudit

//////////////////////////////////////////////
function printForm()
{
   var prtContent = document.getElementById('frmAudit');
   ifrm=document.getElementById('printIframe');
   var oDoc = (ifrm.contentWindow || ifrm.contentDocument);
   if (oDoc.document) 
     oDoc = oDoc.document;
   
   oDoc.open();
   oDoc.write('&lt;link href="DashboardStyles.css" rel="stylesheet" type="text/css""&gt;&lt;/link&gt;&lt;link href="DashboardLocalizedStyles.css" rel="stylesheet" type="text/css""&gt;&lt;/link&gt;');
   oDoc.write('&lt;body onload="self.focus(); self.print();"&gt;');
   oDoc.write(prtContent.innerHTML + "&lt;/body&gt;");  
   oDoc.close();

}
</script>
<body style="margin: 5px;">
<!-- hidden iframe used to print contents -->
<iframe id="printIframe" title="PrintAudit" style="height:0px;width:0px;border:0px;"></iframe>
<form name="frmAudit" ID="frmAudit" onSubmit="return false" target="">

<!-- Save changes to freeze status and description -->
<xsl:call-template name="tSaveChanges"/>

<!-- // Legend (Top) // -->
<xsl:if test="/Report/@rptShowLegend='Y'">
<!-- Purdue Localization C. Karle 12/2/2014 This line originally commented out to not display
     legend at the top.  We want legend to display at top and bottom -->
     <xsl:call-template name="tLegend"/>
</xsl:if>

<!-- // School Header // -->
<xsl:call-template name="tSchoolHeader"/>

<!-- // Student Header // -->
<xsl:if test="/Report/@rptShowStudentHeader='Y'">
<xsl:call-template name="tStudentHeader"/>
</xsl:if>

<!-- // Progress Bar // -->
<xsl:if test="/Report/@rptShowProgressBar='Y'">
<xsl:call-template name="tProgressBar"/>
<br />
</xsl:if>

<!-- // Student Alerts // -->
<xsl:if test="/Report/@rptShowStudentAlerts='Y'">
<xsl:call-template name="tStudentAlerts"/>
</xsl:if>

<!-- Output all the block/rule information -->
<xsl:call-template name="tBlocks" />

<!-- //////////////////////////////////////////////////////////////////////// -->
<!-- Sections:  Fallthrough, Insufficient, Inprogress, OTL (Not Counted, aka Over-the-Limit) 
     (these are all tables that are not nested) -->
<!-- //////////////////////////////////////////////////////////////////////// -->
<br/>

<!--//////////////////////////////////////////////////// 
   // Placeholder Section for planner audits    -->
   	<xsl:if test="/Report/Audit/Placeholders/Placeholder">
      <xsl:call-template name="tSectionPlaceholders"/>
   </xsl:if> 

<!--//////////////////////////////////////////////////// 
   // Fallthrough Section                    -->
   <xsl:if test="/Report/@rptShowFallThroughSection='Y'">
      <xsl:call-template name="tSectionTemplate">
      <xsl:with-param name="pSectionType" select="Fallthrough" />
      <xsl:with-param name="pSectionLabel" select="$LabelFallthrough" />
      </xsl:call-template>
   </xsl:if> 

<!--//////////////////////////////////////////////////// 
   // Insufficient Section                   -->
   <xsl:if test="/Report/@rptShowInsufficientSection='Y'">
      <xsl:call-template name="tSectionTemplate">
      <xsl:with-param name="pSectionType" select="Insufficient" />
      <xsl:with-param name="pSectionLabel" select="$LabelInsufficient" />
      </xsl:call-template>
   </xsl:if> 

<!--//////////////////////////////////////////////////// 
   // Inprogress Section (aka In-Progress)         -->
    <xsl:if test="/Report/@rptShowInProgressSection='Y'">
      <xsl:call-template name="tSectionInprogress">
      </xsl:call-template>
   </xsl:if> 

<!--//////////////////////////////////////////////////// 
   // Not Counted Section (aka OTL, Over-the-limit) -->
    <xsl:if test="/Report/@rptShowOverTheLimitSection='Y'">
      <xsl:call-template name="tSectionTemplate">
      <xsl:with-param name="pSectionType" select="OTL" />
      <xsl:with-param name="pSectionLabel" select="$LabelOTL" />
      </xsl:call-template>
    </xsl:if> 

<!--//////////////////////////////////////////////////// 
    // Split Credits Section                     -->
    <xsl:if test="/Report/@rptShowSplitCreditsSection='Y'">
      <xsl:call-template name="tSectionTemplate">
      <xsl:with-param name="pSectionType" select="SplitCredits" />
      <xsl:with-param name="pSectionLabel" select="$LabelSplitCredits" />
      </xsl:call-template>
    </xsl:if> 


<!-- //////////////////////////////////////////////////////////////////////// -->
<!-- // END Sections -->
<!-- //////////////////////////////////////////////////////////////////////// -->

<br/>

<!-- //////////////////////////////////////////////////////////////////////// -->
<!-- Exceptions -->
<!-- //////////////////////////////////////////////////////////////////////// -->
<xsl:if test="/Report/@rptShowExceptionsSection='Y'">
<xsl:if test="ExceptionList/Exception">
   <xsl:call-template name="tSectionExceptions"/>
</xsl:if> 
</xsl:if> 

<!-- //////////////////////////////////////////////////////////////////////// -->
<!-- Notes -->
<!-- //////////////////////////////////////////////////////////////////////// -->
<xsl:if test="/Report/@rptShowNotesSection='Y'">
<xsl:if test="Notes/Note">
   <xsl:call-template name="tSectionNotes"/>
</xsl:if> 
</xsl:if> 

<!-- //////////////////////////////////////////////////////////////////////// -->
<!-- Legend (Bottom) -->
<!-- //////////////////////////////////////////////////////////////////////// -->
<xsl:if test="/Report/@rptShowLegend='Y'">
   <xsl:call-template name="tLegend"/>
</xsl:if> 

<!-- //////////////////////////////////////////////////////////////////////// -->
<!-- Disclaimer (Bottom)-->
<!-- //////////////////////////////////////////////////////////////////////// -->
<xsl:if test="/Report/@rptShowDisclaimer='Y'">
   <xsl:call-template name="tDisclaimer"/>
</xsl:if> 

<!-- Refresh student context area with this data -->
<xsl:call-template name="tRefreshStudentData" />
<!-- Enable audit buttons in SD2AUDCON -->
<xsl:call-template name="tToggleButtons" />
</form>

</body>
</html>
</xsl:template> <!-- match=Audit -->

<!-- If we don't get an Audit tree we should get an Error node -->
<xsl:template match="Error">
<html>
   <link rel="stylesheet" href="DashboardStyles.css" type="text/css" /><link rel="stylesheet" href="DashboardLocalizedStyles.css" type="text/css" />
<body>
<form name="frmAudit" ID="frmAudit">
  <span class="ErrorMessage"> 
  <xsl:choose>
   <xsl:when test="@Status='1234'"> 
   </xsl:when>
   <xsl:otherwise>
    Status = <xsl:value-of select="@Status"/>
   <br />
    <xsl:value-of select="@WhatMessage"/>
   <br />
    <xsl:value-of select="@ActionMessage"/>
   </xsl:otherwise>
  </xsl:choose>
  </span>
<xsl:call-template name="tToggleButtons" />
</form>
</body>
</html>
</xsl:template> <!-- match=Error -->

<xsl:template name="tToggleButtons">
<xsl:for-each select="/Report/ReloadButtons">
<script type="text/javascript" language="JavaScript">
try
{
   top.frSelection.ToggleButtons("on");   
}
catch (err)
{
   // this is fine -- it just means the top frame does not exist
}
</script>
</xsl:for-each>
</xsl:template>


<xsl:template name="tRefreshStudentData">
<xsl:for-each select="/Report/StudentData" >
<script  type="text/javascript" language="JavaScript">

function FindCode (sPicklistArray, sCodeToFind)
{
   var sReturnValue = sCodeToFind;

   for ( iSearchIndex = 0 ; iSearchIndex &lt; sPicklistArray.length ; iSearchIndex++ )
   {
      if (sPicklistArray[iSearchIndex].code == sCodeToFind)
      {
         sReturnValue = sPicklistArray[iSearchIndex].literal;
         break;
      }
   }
   return sReturnValue;
}

function Update_oViewClass (oStudentToUpdate, sMajors, sLevels, sDegrees)
{
   /*
      [0] = sort name
      [1] = name
      [3] = ID
      [4] = name
      [5] = degree short literal (list separated with space)
      [6] = major literal (list separated with space)
      [7] = school literal (list separated with space)
      [8] = level literal (list separated with space)
   */
      
   oStudentToUpdate[1] = "<xsl:value-of select='PrimaryMst/@Name' />";
   oStudentToUpdate[3] = "<xsl:value-of select='PrimaryMst/@Id' />";
   oStudentToUpdate[4] = "<xsl:value-of select='PrimaryMst/@Name' />";

      /*alert( "myPreviousDegree = " + oStudentToUpdate[5] + "\n" + 
            "myPreviousLevel  = " + oStudentToUpdate[8] + "\n" + 
            "myPreviousMajor  = " + oStudentToUpdate[6] + "\n");*/
   oStudentToUpdate[5] = '';
   oStudentToUpdate[6] = '';
   oStudentToUpdate[8] = '';

   <xsl:for-each select="GoalDtl">
      /* get the degree code, major code, school code, and level code. */     
      myDegree = FindCode (sDegrees, "<xsl:value-of select='@Degree' />");
      myLevel = FindCode (sLevels, "<xsl:value-of select='@StuLevel' />");
      thisDegree = "<xsl:value-of select='@Degree' />";
      myMajor = "";
      <xsl:for-each select="../GoalDataDtl[@GoalCode='MAJOR']">
         if (thisDegree == "<xsl:value-of select='@Degree' />" &amp;&amp; myMajor == "")
         {
            myMajor = FindCode (sMajors, "<xsl:value-of select='@GoalValue' />");
         }
      </xsl:for-each>
      oStudentToUpdate[5] += top.frControl.Trim(myDegree) + ' ';
      oStudentToUpdate[6] += top.frControl.Trim(myMajor)  + ' ';
      oStudentToUpdate[8] += top.frControl.Trim(myLevel)  + ' ';
      /*alert("Degree Info after:\n" + 
            "myPreviousDegree = " + oStudentToUpdate[5] + "\n" + 
            "myPreviousLevel  = " + oStudentToUpdate[8] + "\n" + 
            "myPreviousMajor  = " + oStudentToUpdate[6] + "\n");*/
   </xsl:for-each>
   /*
   oStudentToUpdate[5] = '';
   oStudentToUpdate[6] = '';
   oStudentToUpdate[7] = '';
   oStudentToUpdate[8] = '';
   */
   return oStudentToUpdate;

}
function Update_studentArray (aStudentToUpdate, sMajors, sLevels, sDegrees)
{
/*
   this.degree = Trim(degree);
   this.degreelit = Trim(degreelit);
   this.school = Trim(school);
   this.majorlit = Trim(majorlit);
   this.level = Trim(level);
   this.degreeinterest = Trim(degreeinterest);
*/
   aStudentToUpdate.name = "<xsl:value-of select='PrimaryMst/@Name' />";

   sRefreshDate = top.frControl.FormatRefreshDate("<xsl:value-of select='PrimaryMst/@BridgeDate' />");
   sRefreshTime = top.frControl.FormatRefreshTime("<xsl:value-of select='PrimaryMst/@BridgeTime' />");
   aStudentToUpdate.refreshdate = sRefreshDate + " at " + sRefreshTime;
   aStudentToUpdate.refreshdate.title = "";
   
   myAuditId = '<xsl:value-of select="/Report/Audit/AuditHeader/@Audit_id" />';
   if (myAuditId.substring(0,1) == "A") // if it is a real audit then update the auditdate otherwise do not.
   {
      aStudentToUpdate.auditdate = "Today";
   }
   
   aStudentToUpdate.degrees.length = 0;

   <xsl:for-each select="GoalDtl">
   myDegree = FindCode (sDegrees, "<xsl:value-of select='@Degree' />");
   myLevel  = FindCode (sLevels, "<xsl:value-of select='@StuLevel' />");
   thisDegree = "<xsl:value-of select='@Degree' />";
   myMajor = "";
   <xsl:for-each select="../GoalDataDtl[@GoalCode='MAJOR']">
      if (thisDegree == "<xsl:value-of select='@Degree' />" &amp;&amp; myMajor == "")
      {
         myMajor = FindCode (sMajors, "<xsl:value-of select='@GoalValue' />");
      }
   </xsl:for-each>
   aStudentToUpdate.degrees[aStudentToUpdate.degrees.length] = 
         new top.frControl.DegreeEntry("<xsl:value-of select='@Degree' />", 
                                myDegree, 
                                "<xsl:value-of select='@School' />", myMajor, myLevel, "");
   </xsl:for-each>
} // update_studentarray

<xsl:if test="PrimaryMst">
  //alert('"<xsl:value-of select="PrimaryMst/@Name" />" was successfully refreshed.');                                 
  var moz;
  moz = (typeof document.implementation != 'undefined') &amp;&amp; 
        (typeof document.implementation.createDocument != 'undefined');
/*
  if (moz)
    {
    thisForm   = top.frControl.document.getElementById("formCallScript");
    thisForm.elements['PRELOADEDPLAN'].value = "<xsl:value-of select="/Save/@PreloadedPlan" />"
    thisForm.elements['RELOADSEP'].value = "FALSE";
   }
  else // ie etc
    {
    top.frControl.frmCallScript.PRELOADEDPLAN.value = '<xsl:value-of select="/Save/@PreloadedPlan" />';
    top.frControl.frmCallScript.RELOADSEP.value = "FALSE";                      
    }
*/
   //alert("top.frControl.studentArray.length = " + top.frControl.studentArray.length);
  var sRefreshedStudentID   = '<xsl:value-of select="PrimaryMst/@Id" />';
  var sRefreshedStudentName = "<xsl:value-of select='PrimaryMst/@Name' />";

   //alert('Student just refreshed = ' + sRefreshedStudentID + '\n' + sRefreshedStudentName);

   //alert("top.frControl.sa.length = " + top.frControl.sa.length);

   var bOnlySimpleSearch = true;
   if (top.frControl.oViewClass != undefined)
   {
      bOnlySimpleSearch = false;
   }
   if (!bOnlySimpleSearch)
   {
      var oStudentList = top.frControl.oViewClass;
      var iStudentListLength = oStudentList.length;

      //alert("iStudentListLength = " + iStudentListLength);

      for ( iStudentArrayIndex = 0; iStudentArrayIndex &lt; iStudentListLength ; iStudentArrayIndex++ )
      {
         var bIsDefined = true;
         var i = 0;
         while (bIsDefined)
         {
            if (oStudentList[iStudentArrayIndex][i] != undefined)
            {
               if (oStudentList[iStudentArrayIndex][i] == sRefreshedStudentID)
               {
                  //alert("I found " + sRefreshedStudentID + " in my list!");
                  top.frControl.oViewClass[iStudentArrayIndex] = Update_oViewClass (
                                 top.frControl.oViewClass[iStudentArrayIndex],
                                 top.frControl.sMajorPicklist,
                                 top.frControl.sLevelPicklist,
                                 top.frControl.sDegreePicklist);
                  //bIsDefined = false;
               }
               //alert("oStudentList[" + iStudentArrayIndex + "][" + i + "] = " + oStudentList[iStudentArrayIndex][i]);
            }
            else
            {
               bIsDefined = false;
            }
            i++;
         }
      }
   }

   var aStudentArray = top.frControl.studentArray;
   var iStudentArrayLength = aStudentArray.length;
   var iCurrentDegreeIndex = top.frControl.oDegreeList.selectedIndex;

   //alert("iStudentArrayLength = " + iStudentArrayLength);
   for ( iStudentArrayIndex = 0; iStudentArrayIndex &lt; iStudentArrayLength ; iStudentArrayIndex++ )
   {
      if (aStudentArray[iStudentArrayIndex].id == sRefreshedStudentID)
      {
         //alert("I found " + sRefreshedStudentID + " in my second list!");
         Update_studentArray (top.frControl.studentArray[iStudentArrayIndex],
                     top.frControl.sMajorPicklist,
                     top.frControl.sLevelPicklist,
                     top.frControl.sDegreePicklist)
      }
      //alert("aStudentArray[" + iStudentArrayIndex + "].auditdate = " + aStudentArray[iStudentArrayIndex].auditdate);
   }
    // Set student context but do not reload body (reason for "false")
    // Keep the currently select degree as the one selected
    top.frControl.SetStudent(false, iCurrentDegreeIndex); 

</xsl:if>

</script>
</xsl:for-each > <!-- StudentData node -->

<script type="text/javascript" language="JavaScript">
<xsl:if test="not(/Report/StudentData/PrimaryMst)">
  if (typeof(top.frControl) != "undefined")
   {
   myAuditId = '<xsl:value-of select="/Report/Audit/AuditHeader/@Audit_id" />';
   // if it is a real audit then update the auditdate otherwise do not
   if (myAuditId.substring(0,1) == "A") 
    {
    sTodaysDate = top.frControl.GetCurrentDate(); // mm/dd/ccyy or whatever the format is
    sAuditDate = top.frControl.FormatDate ("<xsl:value-of select="concat(/Report/Audit/AuditHeader/@DateYear,/Report/Audit/AuditHeader/@DateMonth,/Report/Audit/AuditHeader/@DateDay)" />");
    //sAuditMonth = '<xsl:value-of select="/Report/Audit/AuditHeader/@DateMonth" />';
    //sAuditDay   = '<xsl:value-of select="/Report/Audit/AuditHeader/@DateDay"   />';
    //sAuditYear  = '<xsl:value-of select="/Report/Audit/AuditHeader/@DateYear"  />';
    //sAuditDate = sAuditMonth + '/' + sAuditDay + '/' + sAuditYear;
    // If this audit was run today then show its time; otherwise the date of the last
    // run audit should already be displaying and there is no reason to show this old date
    if (sAuditDate == sTodaysDate)
      {
      //sAuditHour  = '<xsl:value-of select="/Report/Audit/AuditHeader/@TimeHour"  />';
      //sAuditMin   = '<xsl:value-of select="/Report/Audit/AuditHeader/@TimeMinute"/>';
      //sDisplayDate = sAuditHour + ':' + sAuditMin;
      sDisplayDate = 'Today';
      // Update the display date at the top with this new date
      top.frControl.document.frmHoldFields.LastAudit.value = sDisplayDate;
      }
    }
   } // frcontrol != undefined
</xsl:if> <!-- not primarymst -->
</script>

</xsl:template>

<xsl:template name="tCreditsLiteral"> <!-- 1.19 -->
<xsl:choose>
 <xsl:when test="@Credits = 1">
  <xsl:value-of select="normalize-space(/Report/@rptCreditSingular)" />
 </xsl:when>
 <xsl:otherwise>
  <xsl:value-of select="normalize-space(/Report/@rptCreditsLiteral)" />
 </xsl:otherwise>
</xsl:choose>
</xsl:template>

<xsl:include href="AuditDisclaimer.xsl" />

<xsl:template name="tDisclaimer"> 
<br/>
   <table border="0" cellspacing="1" cellpadding="0" width="100%" class="Blocks">
      <tr>
      <td colspan="20">
      <table border="0" cellspacing="0" cellpadding="0" width="100%" class="BlockHeader">
         <tr>
            <td class="BlockHeader" colspan="1" rowspan="2" valign="middle" nowrap="true">
<!-- Purdue localization 8/2017 Add s to Disclaimer -->
               &#160;Disclaimers
            </td>
         </tr>
      </table>
      </td>
      </tr>
      <tr>
         <td class="DisclaimerText">
            <xsl:value-of select="$vDisclaimerText" />
         </td>
      </tr>

   </table>
</xsl:template> 

<xsl:template name="tTitleForPrintedAudit">
<title>
   <xsl:choose>
      <xsl:when test="/Report/@rptCFG020AuditTitleStyle='A'">
         Ellucian Degree Works 
      </xsl:when>
      <xsl:when test="/Report/@rptCFG020AuditTitleStyle='B'">
         Degree Works <xsl:value-of select="/Report/@rptReportName" /> 
      </xsl:when>
      <xsl:when test="/Report/@rptCFG020AuditTitleStyle='C'">
         <xsl:value-of select="/Report/@rptInstitutionName" />:
         <xsl:value-of select="/Report/@rptReportName" /> 
      </xsl:when>
      <xsl:when test="/Report/@rptCFG020AuditTitleStyle='D'">
         <xsl:value-of select="/Report/@rptReportName" /> 
         for 
         <xsl:value-of select="/Report/Audit/AuditHeader/@Stu_name" /> 
      </xsl:when>
      <xsl:when test="/Report/@rptCFG020AuditTitleStyle='E'">
         <xsl:value-of select="/Report/@rptReportName" /> 
         for 
         <xsl:value-of select="/Report/Audit/AuditHeader/@Stu_name" /> 
         -
         <xsl:call-template name="tStudentID" />
      </xsl:when>
      <xsl:when test="/Report/@rptCFG020AuditTitleStyle='F'">
         <xsl:value-of select="/Report/@rptInstitutionName" />:
         <xsl:value-of select="/Report/@rptReportName" /> 
         for 
         <xsl:value-of select="/Report/Audit/AuditHeader/@Stu_name" /> 
      </xsl:when>
      <xsl:when test="/Report/@rptCFG020AuditTitleStyle='G'">
         <xsl:value-of select="/Report/@rptInstitutionName" />:
         <xsl:value-of select="/Report/@rptReportName" /> 
         for 
         <xsl:value-of select="/Report/Audit/AuditHeader/@Stu_name" /> 
         -
         <xsl:call-template name="tStudentID" />
      </xsl:when>
      <xsl:otherwise><!-- "E" is default -->
         <xsl:value-of select="/Report/@rptReportName" /> 
         for 
         <xsl:value-of select="/Report/Audit/AuditHeader/@Stu_name" /> 
         -
         <xsl:call-template name="tStudentID" />
      </xsl:otherwise>
   </xsl:choose>
</title>
</xsl:template>
<xsl:template name="tSchoolHeader"> 
<!-- //////////////////////////////////////////////////////////////////////// -->
<!-- School Name: Using Text -->
<!-- //////////////////////////////////////////////////////////////////////// -->
<table border="0" cellspacing="0" cellpadding="0" width="100%" class="SchoolName">                                     
   <tr>
      <td align="left" valign="top">
      <table border="0" cellspacing="0" cellpadding="4" width="100%">       
         <tr>
            <td align="center" valign="middle">
            <span class="SchoolName"><xsl:value-of select="/Report/@rptInstitutionName" />
            </span>
            </td>
         </tr>
      </table>
      </td>
   </tr>
</table>
<!-- //////////////////////////////////////////////////////////////////////// -->
<!-- FOR_CUSTOMIZING:SCHOOLNAME -->
<!-- //////////////////////////////////////////////////////////////////////// -->
<!-- School Name: To use an image, replace the <img src> tag below.
<table border="0" cellspacing="0" cellpadding="0" width="100%" class="SchoolName">                                     
   <tr>
      <td align="left" valign="top">
      <table border="0" cellspacing="0" cellpadding="4" width="100%">
         <tr>
            <td align="center" valign="middle">
            <img src="Images_DG2/Icon_DegreeWorks.gif" />
            <br />
            </td>
         </tr>
      </table>
      </td>
   </tr>
</table> -->
<!-- //////////////////////////////////////////////////////////////////////// -->
</xsl:template> 

<!-- //////////////////////////////////////////////////////////////////////// -->
<!-- STUDENT ALERTS -->
<!-- //////////////////////////////////////////////////////////////////////// -->
<xsl:template name="tStudentAlerts"> 
<!-- If an ALERT1 Report node exists -->
<xsl:if test="/Report/Audit/Deginfo/Report/@Code='ALERT1'"> 
<br />
<table border="0" cellspacing="0" cellpadding="0" width="60%" align="center" class="AuditTable">

	<tr>
		<td class="AuditHeadBorderDark">
		</td>
	</tr>
	<tr>
		<td class="AuditHeadBorderLight">
		</td>
	</tr>

	<tr>                                                                          
		<td align="left" valign="top">                                               
		<table border="0" cellspacing="1" cellpadding="2" width="100%">
			<tr>
				<td align="center" valign="middle" class="StuHead">
				<span class="StuHeadTitle"> 
					<xsl:copy-of select="$LabelAlertsReminders" />
				</span>
				<br />
				</td>
			</tr>
		</table>
		</td>
	</tr>
	
	<tr>
		<td class="AuditHeadBorderLight">
		</td>
	</tr>
	
	<tr>
		<td class="AuditHeadBorderDark">
		</td>
	</tr>
	
	<tr>
		<td>
		<table class="Inner" cellspacing="1" cellpadding="3" border="0" width="100%">

			<xsl:if test="/Report/Audit/Deginfo/Report/@Code='ALERT1'"> 
			  <!-- xxxxxxxxxxx NEXT ROW xxxxxxxxxxx -->
			  <tr class="StuTableTitle">
				<td class="StuTableData" >
				<img src="Images_DG2/Arrow_Right.gif"  ondragstart="window.event.returnValue=false;"/>&#160;
					<xsl:for-each select="/Report/Audit/Deginfo/Report[@Code='ALERT1']">
						<xsl:value-of select="@Value" />
					</xsl:for-each>
				</td>
			  </tr>
			</xsl:if>
			<xsl:if test="/Report/Audit/Deginfo/Report/@Code='ALERT2'"> 
			  <!-- xxxxxxxxxxx NEXT ROW xxxxxxxxxxx -->
			  <tr class="StuTableTitle">
				<td class="StuTableData" >
				<img src="Images_DG2/Arrow_Right.gif"  ondragstart="window.event.returnValue=false;"/>&#160;
					<xsl:for-each select="/Report/Audit/Deginfo/Report[@Code='ALERT2']">
						<xsl:value-of select="@Value" />
					</xsl:for-each>
				</td>
			  </tr>
			</xsl:if>
			<xsl:if test="/Report/Audit/Deginfo/Report/@Code='ALERT3'"> 
			  <!-- xxxxxxxxxxx NEXT ROW xxxxxxxxxxx -->
			  <tr class="StuTableTitle">
				<td class="StuTableData" >
				<img src="Images_DG2/Arrow_Right.gif"  ondragstart="window.event.returnValue=false;"/>&#160;
					<xsl:for-each select="/Report/Audit/Deginfo/Report[@Code='ALERT3']">
						<xsl:value-of select="@Value" />
					</xsl:for-each>
				</td>
			  </tr>
			</xsl:if>
			<xsl:if test="/Report/Audit/Deginfo/Report/@Code='ALERT4'"> 
			  <!-- xxxxxxxxxxx NEXT ROW xxxxxxxxxxx -->
			  <tr class="StuTableTitle">
				<td class="StuTableData" >
				<img src="Images_DG2/Arrow_Right.gif"  ondragstart="window.event.returnValue=false;"/>&#160;
					<xsl:for-each select="/Report/Audit/Deginfo/Report[@Code='ALERT4']">
						<xsl:value-of select="@Value" />
					</xsl:for-each>
				</td>
			  </tr>
			</xsl:if>
			<xsl:if test="/Report/Audit/Deginfo/Report/@Code='ALERT5'"> 
			  <!-- xxxxxxxxxxx NEXT ROW xxxxxxxxxxx -->
			  <tr class="StuTableTitle">
				<td class="StuTableData" >
				<img src="Images_DG2/Arrow_Right.gif"  ondragstart="window.event.returnValue=false;"/>&#160;
					<xsl:for-each select="/Report/Audit/Deginfo/Report[@Code='ALERT5']">
						<xsl:value-of select="@Value" />
					</xsl:for-each>
				</td>
			  </tr>
			</xsl:if>
			<xsl:if test="/Report/Audit/Deginfo/Report/@Code='ALERT6'"> 
			  <!-- xxxxxxxxxxx NEXT ROW xxxxxxxxxxx -->
			  <tr class="StuTableTitle">
				<td class="StuTableData" >
				<img src="Images_DG2/Arrow_Right.gif"  ondragstart="window.event.returnValue=false;"/>&#160;
					<xsl:for-each select="/Report/Audit/Deginfo/Report[@Code='ALERT6']">
						<xsl:value-of select="@Value" />
					</xsl:for-each>
				</td>
			  </tr>
			</xsl:if>
			<xsl:if test="/Report/Audit/Deginfo/Report/@Code='ALERT7'"> 
			  <!-- xxxxxxxxxxx NEXT ROW xxxxxxxxxxx -->
			  <tr class="StuTableTitle">
				<td class="StuTableData" >
				<img src="Images_DG2/Arrow_Right.gif"  ondragstart="window.event.returnValue=false;"/>&#160;
					<xsl:for-each select="/Report/Audit/Deginfo/Report[@Code='ALERT7']">
						<xsl:value-of select="@Value" />
					</xsl:for-each>
				</td>
			  </tr>
			</xsl:if>
			<xsl:if test="/Report/Audit/Deginfo/Report/@Code='ALERT8'"> 
			  <!-- xxxxxxxxxxx NEXT ROW xxxxxxxxxxx -->
			  <tr class="StuTableTitle">
				<td class="StuTableData" >
				<img src="Images_DG2/Arrow_Right.gif"  ondragstart="window.event.returnValue=false;"/>&#160;
					<xsl:for-each select="/Report/Audit/Deginfo/Report[@Code='ALERT8']">
						<xsl:value-of select="@Value" />
					</xsl:for-each>
				</td>
			  </tr>
			</xsl:if>
			<xsl:if test="/Report/Audit/Deginfo/Report/@Code='ALERT9'"> 
			  <!-- xxxxxxxxxxx NEXT ROW xxxxxxxxxxx -->
			  <tr class="StuTableTitle">
				<td class="StuTableData" >
				<img src="Images_DG2/Arrow_Right.gif"  ondragstart="window.event.returnValue=false;"/>&#160;
					<xsl:for-each select="/Report/Audit/Deginfo/Report[@Code='ALERT9']">
						<xsl:value-of select="@Value" />
					</xsl:for-each>
				</td>
			  </tr>
			</xsl:if>
			<xsl:if test="/Report/Audit/Deginfo/Report/@Code='ALERT10'"> 
			  <!-- xxxxxxxxxxx NEXT ROW xxxxxxxxxxx -->
			  <tr class="StuTableTitle">
				<td class="StuTableData" >
				<img src="Images_DG2/Arrow_Right.gif"  ondragstart="window.event.returnValue=false;"/>&#160;
					<xsl:for-each select="/Report/Audit/Deginfo/Report[@Code='ALERT10']">
						<xsl:value-of select="@Value" />
					</xsl:for-each>
				</td>
			  </tr>
			</xsl:if>
	  </table>
	</td></tr>	
</table>
<br />
</xsl:if>
</xsl:template> 
<!-- //////////////////////////////////////////////////////////////////////// -->
<!-- STUDENT ALERTS END -->
<!-- //////////////////////////////////////////////////////////////////////// -->

<xsl:template name="tProgressBar"> 
<!-- //////////////////////////////////////////////////////////////////////// -->
<!-- PROGRESS BAR  -->
<!-- //////////////////////////////////////////////////////////////////////// -->
  <!-- /Report/Audit/AuditHeader/@Per_complete contains the percent complete -->
    <br />
    <center>
    <span class="ProgressTitle"> 
      <xsl:copy-of select="$LabelProgressBar" />
    </span>
   <xsl:if test="$vProgressBarPercent='Y'">
    <table cellpadding="0" cellspacing="1" class="ProgressTable" >
    <xsl:attribute name="title"><xsl:value-of select="$vProgressBarRulesText" /></xsl:attribute>
       <tr>
         <td class="ProgressSubTitle">Requirements
        </td>
          <td>
        <xsl:attribute name="class">ProgressBar</xsl:attribute>
          <xsl:if test="/Report/Audit/AuditHeader/@Per_complete = 0">
           <xsl:attribute name="width"> 5%
           </xsl:attribute>
          </xsl:if>
          <xsl:if test="/Report/Audit/AuditHeader/@Per_complete = 100">
           <xsl:attribute name="width"> 100%
           </xsl:attribute>
         </xsl:if>
          <xsl:if test="/Report/Audit/AuditHeader/@Per_complete &gt; 0">
           <xsl:if test="/Report/Audit/AuditHeader/@Per_complete &lt; 5">
           <xsl:attribute name="width"> 5%
           </xsl:attribute>
         </xsl:if>
         <xsl:if test="/Report/Audit/AuditHeader/@Per_complete &lt; 100 and
                     /Report/Audit/AuditHeader/@Per_complete &gt;= 5">
              <xsl:attribute name="width">
              <xsl:value-of select="/Report/Audit/AuditHeader/@Per_complete" />%
              </xsl:attribute>
          </xsl:if>
          </xsl:if>
         <center>
                   <xsl:value-of select='format-number(/Report/Audit/AuditHeader/@Per_complete, "0")' />% <!-- was #.0 but Mike wants no decimals -->
         </center>
          </td>
        <td>
       <xsl:if test="/Report/Audit/AuditHeader/@Per_complete = 100">
      </xsl:if>
       <xsl:if test="/Report/Audit/AuditHeader/@Per_complete &lt; 100">
              &#160;
      </xsl:if>
          </td>
     </tr>
    </table>
    </xsl:if> <!-- vProgressBarPercent -->
    </center>

   <xsl:if test="$vProgressBarCredits='Y'">
    <xsl:choose>
     <xsl:when test="/Report/Audit/Block[1]/Header/Qualifier[@Node_type='4121']">

      <xsl:variable name="vOverallCreditsRequired">
       <xsl:call-template name="tCreditsRequired" />
      </xsl:variable>

      <xsl:variable name="vOverallCreditsApplied">
        <xsl:choose>
        <xsl:when test="/Report/Audit/Block[1]/Header/Qualifier/CREDITSAPPLIEDTOWARDSDEGREE">
         <xsl:value-of select="/Report/Audit/Block[1]/Header/Qualifier/CREDITSAPPLIEDTOWARDSDEGREE/@Credits" />
        </xsl:when>
        <xsl:otherwise>
         <xsl:value-of select="/Report/Audit/Block[1]/@Credits_applied" />
        </xsl:otherwise>
        </xsl:choose>
      </xsl:variable>
      
      <xsl:variable name="vOverallCreditsPercentComplete">
		<xsl:choose>
		<xsl:when test="100 * ($vOverallCreditsApplied div $vOverallCreditsRequired) &gt; 100">100</xsl:when>
		<xsl:otherwise>
			<xsl:value-of select="100 * ($vOverallCreditsApplied div $vOverallCreditsRequired)" />
		</xsl:otherwise>
		</xsl:choose>
      </xsl:variable>
      
      <br />
      <center>
      <table cellpadding="0" cellspacing="1" class="ProgressTable" >
       <xsl:attribute name="title"><xsl:value-of select="$vProgressBarCreditsText" />
         <xsl:if test="/Report/Audit/Block[1]/Header/Qualifier/CREDITSAPPLIEDTOWARDSDEGREE"> 
            (excluding excess electives)</xsl:if></xsl:attribute>
       <tr>
         <td class="ProgressSubTitle"><xsl:call-template name="tCreditsLiteral"/>
        </td>
          <td>
        <xsl:attribute name="class">ProgressBar</xsl:attribute>
          <xsl:if test="$vOverallCreditsPercentComplete = 0">
           <xsl:attribute name="width"> 5%
           </xsl:attribute>
          </xsl:if>
          <xsl:if test="$vOverallCreditsPercentComplete &gt; 99.99">
           <xsl:attribute name="width"> 100%
           </xsl:attribute>
         </xsl:if>
          <xsl:if test="$vOverallCreditsPercentComplete &gt; 0">
          <xsl:if test="$vOverallCreditsPercentComplete &lt; 100">
              <xsl:attribute name="width">
              <xsl:copy-of select="$vOverallCreditsPercentComplete" />%
              </xsl:attribute>
          </xsl:if>
          </xsl:if>
         <center>
                   <xsl:copy-of select='format-number($vOverallCreditsPercentComplete, "0")' />% <!-- was #.0 but Mike wants no decimals -->
         </center>
          </td>
        <td>
       <xsl:if test="$vOverallCreditsPercentComplete = 100">
       </xsl:if>
       <xsl:if test="$vOverallCreditsPercentComplete &lt; 100">
              &#160;
      </xsl:if>
        </td>
     </tr>
    </table>
    </center>
   </xsl:when>
   <xsl:otherwise /> <!-- do nothing. No credits rule in the starter block so no data to use to calculate credits pct complete -->
   </xsl:choose>
   </xsl:if> <!-- vProgressBarCredits -->

<!-- //////////////////////////////////////////////////////////////////////// -->
<!-- PROGRESS BAR END -->
<!-- //////////////////////////////////////////////////////////////////////// -->
</xsl:template> 

<xsl:template name="tSectionPlaceholders">
   <table border="0" cellspacing="1" cellpadding="0" width="100%" class="xBlocks">
      <tr>
         <td colspan="20">
         <table border="0" cellspacing="0" cellpadding="0" width="100%" class="BlockHeader">
         <tr >
            <td class="BlockHeader" colspan="1" rowspan="2" valign="middle" nowrap="true">
               &#160;
               <xsl:copy-of select="$LabelPlaceholders" />
            </td>
         </tr>
      </table>
      </td>
   </tr>

   <xsl:for-each select="/Report/Audit/Placeholders/Placeholder">
   <tr>
        <xsl:if test="position() mod 2 = 0">
         <xsl:attribute name="class">CourseAppliedRowAlt</xsl:attribute>
        </xsl:if>
        <xsl:if test="position() mod 2 = 1">
         <xsl:attribute name="class">CourseAppliedRowWhite</xsl:attribute>
        </xsl:if>

      <td class="SectionCourseTitle" >
        <xsl:value-of select="@Description"/> 
       </td>
      <td class="SectionCourseTitle" >
        <xsl:value-of select="@Value"/> 
       </td>
   </tr>
   </xsl:for-each>
   </table>
</xsl:template>

<!-- tSectionTemplate   exists in AuditSections.xsl -->
<!-- tSectionInprogress exists in AuditSections.xsl -->

<!-- Course grade -->
<xsl:template name="tCourseGrade">
    <xsl:value-of select="@Letter_grade"/> 
    <xsl:text>&#160;</xsl:text> <!-- space --> 
</xsl:template>

<!-- tSectionExceptions is in AuditSections.xsl -->
<!-- tSectionNotes      is in AuditSections.xsl -->

<!-- template tIndentLevel-Advice removed - not used 1.15 -->

<xsl:template name="tStudentID"> 
<xsl:variable name="stu_id"           select="normalize-space(AuditHeader/@Stu_id)"/>
<xsl:variable name="stu_id_length"    select="string-length(normalize-space(AuditHeader/@Stu_id))"/>
<xsl:variable name="fill_asterisks"   select="$stu_id_length"/>
<xsl:variable name="bytes_to_remove"  select="/Report/@rptCFG020MaskStudentID"/>

<xsl:variable name="bytes_to_show"    select="$stu_id_length - $bytes_to_remove"/>
<xsl:variable name="myAsterisks">
<xsl:call-template name="tAsterisks" >
   <xsl:with-param name="bytes_to_remove" select="$bytes_to_remove" />
</xsl:call-template>
</xsl:variable>

<xsl:variable name="formatted_stu_id" />
<xsl:choose>
   <xsl:when test="/Report/@rptCFG020MaskStudentID = 'A'">  
      <xsl:call-template name="tFillAsterisks" >
         <xsl:with-param name="string_length" select="$fill_asterisks" />
      </xsl:call-template>
   </xsl:when>
   <xsl:when test="/Report/@rptCFG020MaskStudentID = 'N'">  
      <xsl:value-of select="AuditHeader/@Stu_id"/>
   </xsl:when>
   <xsl:otherwise>
      <xsl:value-of select="concat($myAsterisks, substring($stu_id, $bytes_to_remove + 1, $bytes_to_show))" />
   </xsl:otherwise>
</xsl:choose>

</xsl:template>

<xsl:template match="FreezeTypes">
<!-- do nothing; we need this so the codes in the UserClass nodes don't get displayed automatically -->
</xsl:template>

<!-- tBlocks template contains the block/rule templates -->
<xsl:include href="AuditBlocks.xsl" />

<!-- Templates for in-progress, fallthrough, insufficient, and over-the-limit -->
<!-- Also contains logic for notes and exceptions -->
<xsl:include href="AuditSections.xsl" />

<!-- tLegend template is in this included xsl; shared by athletic and academic audits -->
<xsl:include href="AuditLegend.xsl" />

<!-- tStudentHeader template is in this included xsl; shared by athletic and fin-aid audits -->
<xsl:include href="AuditStudentHeader.xsl" />

<!-- FormatDate template is in this included xsl -->
<xsl:include href="FormatDate.xsl" />

<!--
<xsl:template name="tFormatNumber">
<xsl:template name="FormatRuleXptDate">   
<xsl:template name="FormatXptDate"> 
<xsl:template name="FormatNoteDate">   
<xsl:template name="globalReplace">
<xsl:template name="tFillAsterisks">
<xsl:template name="tAsterisks">
-->
<!-- Templates for general functionality -->
<xsl:include href="CommonTemplates.xsl" />

</xsl:stylesheet>
