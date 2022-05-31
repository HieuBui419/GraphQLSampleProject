<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BookUpdate</name>
   <tag></tag>
   <elementGuidId>c506fa00-0f37-4faa-aaa1-4622bc69a0fa</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;query\&quot;: \&quot;mutation Mutation { updateBook(id:3, title:\\\&quot;Ansataria\\\&quot;){ id title pageCount } }\&quot;\n}&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;displayText&quot;: &quot;mutation Mutation {\n  updateBook(id:3, title:\&quot;Ansataria\&quot;){\n    id\n    title\n    pageCount\n  }\n}&quot;,
  &quot;displayVariables&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;
}</httpBodyContent>
   <httpBodyType>GraphQL</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>6b9ff494-76f1-4b27-b40c-29ac2cc90fb6</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://katalon-sample-graphql-aut.herokuapp.com/graphql</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
