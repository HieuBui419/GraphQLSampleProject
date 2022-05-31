<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CountryQueryWithVars</name>
   <tag></tag>
   <elementGuidId>13f67d03-c7ef-43a9-9cf8-4c30fdd4265c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;query\&quot;: \&quot;query Query($id: ID!) { country(code: $id) { name native capital emoji currency languages { code name } } }\&quot;,\n  \&quot;variables\&quot;: \&quot;{ \\\&quot;id\\\&quot; : \\\&quot;AU\\\&quot; }\&quot;\n}&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;displayText&quot;: &quot;query Query($id: ID!) {\n  country(code: $id) {\n    name\n    native\n    capital\n    emoji\n    currency\n    languages {\n      code\n      name\n    }\n  }\n}&quot;,
  &quot;displayVariables&quot;: &quot;{\n\t\&quot;id\&quot; : \&quot;AU\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;
}</httpBodyContent>
   <httpBodyType>GraphQL</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>63533f3f-2820-44f9-9379-332579299fb5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://countries.trevorblades.com/graphql</restUrl>
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
