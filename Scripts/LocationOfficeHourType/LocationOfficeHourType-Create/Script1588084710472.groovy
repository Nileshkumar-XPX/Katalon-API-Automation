import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonSlurper as JsonSlurper

Token = WS.sendRequest(findTestObject('Provider-Location_Intersect/LocationOfficeHourType/Token', [('variable') : '']))

def slurper = new JsonSlurper()

def result = slurper.parseText(Token.getResponseBodyContent())

def value = result.access_token

GlobalVariable.AccessToken = value

IdForUpdateDelete = WS.sendRequestAndVerify(findTestObject('Provider-Location_Intersect/LocationOfficeHourType/CREATE LocationOfficeHourType', 
        [('variable') : '']))

def slurper1 = new JsonSlurper()

def result1 = slurper1.parseText(IdForUpdateDelete.getResponseBodyContent())

GlobalVariable.IdForUpdateDelete = result1.providerLocationOperatingScheduleTypeId

